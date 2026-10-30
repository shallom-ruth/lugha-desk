args <- commandArgs(TRUE)
exe <- normalizePath(args[1],winslash='/',mustWork=TRUE)
# Requires jsonlite in the developer's R library; not needed by app users.
library(jsonlite)
out <- normalizePath(args[2],winslash='/',mustWork=TRUE)
d <- fromJSON(args[3])
d <- d[!d$direction %in% c('en-ln','ln-en'),]
# Use ASCII JSON escapes for every non-ASCII character, avoiding R locale conversions.
escape <- function(s)paste(vapply(utf8ToInt(s),function(x)if(x>127)sprintf('\\u%04x',x) else if(x==34)'\\"' else if(x==92)'\\\\' else if(x==10)'\\n' else if(x==9)'\\t' else intToUtf8(x),character(1)),collapse='')
wire <- function(dir,s)paste0('{"direction":"',dir,'","text":"',escape(s),'"}')
cases <- do.call(rbind,lapply(seq_len(nrow(d)),function(i)data.frame(direction=d$direction[i],text=c(d$source[i],paste0(d$source[i],'.'),paste0('  ',d$source[i],'?!  '),chartr('abcdefghijklmnopqrstuvwxyz','ABCDEFGHIJKLMNOPQRSTUVWXYZ',d$source[i]),gsub(' ','  ',d$source[i],fixed=TRUE)),expected=d$target[i],kind='greeting',stringsAsFactors=FALSE)))
miss <- data.frame(direction=c('en-ln','ln-en','en-rw','en-rw','en-rw','en-rw','en-mg'),text=c('Good morning.','Mbote na ntongo.','Good morning, Rehema.','She said Good morning.','Good morning. Hello.','Good\nmorning.','I borrowed a book.'),expected='',kind='model fallback',stringsAsFactors=FALSE)
cases <- rbind(cases,miss)
input <- file.path(out,'r-input.jsonl'); output <- file.path(out,'r-output.jsonl')
lines <- vapply(seq_len(nrow(cases)),function(i)wire(cases$direction[i],cases$text[i]),character(1))
stopifnot(all(as.integer(charToRaw(paste(lines,collapse='\n')))<128L))
for(i in seq_len(nrow(cases)))stopifnot(identical(utf8ToInt(fromJSON(lines[i])$text),utf8ToInt(cases$text[i])))
writeLines(lines,input,useBytes=TRUE)
# exe is supplied by the developer.
# Build folder intentionally has no model/runtime: greeting hits must need neither.
stopifnot(!dir.exists(file.path(dirname(exe),'model')),!dir.exists(file.path(dirname(exe),'runtime')))
old <- Sys.getenv('PATH');Sys.setenv(PATH='',HF_HUB_OFFLINE='1',TRANSFORMERS_OFFLINE='1')
status <- system2(exe,'--session-worker',stdin=input,stdout=output,stderr=file.path(out,'r-stderr.txt'),timeout=60)
Sys.setenv(PATH=old)
r <- Filter(function(x)!is.null(x$result)||!is.null(x$error),lapply(readLines(output,encoding='UTF-8'),fromJSON))
stopifnot(status==0,length(r)==nrow(cases))
cases$passed <- vapply(seq_len(nrow(cases)),function(i)if(cases$kind[i]=='greeting')identical(utf8ToInt(r[[i]]$result),utf8ToInt(cases$expected[i])) else !is.null(r[[i]]$error)&&grepl('runtime is missing',r[[i]]$error,fixed=TRUE),logical(1))
write_json(list(cases=cases,responses=r,exit_status=status),file.path(out,'r-results.json'),pretty=TRUE,auto_unbox=TRUE)
write.csv(cases[,c('direction','kind','passed')],file.path(out,'r-checks.csv'),row.names=FALSE)
capture.output(sessionInfo(),file=file.path(out,'R-session.txt'))
cat(sum(cases$passed),'/',nrow(cases),'R integration checks passed\n')
stopifnot(all(cases$passed))
# Exercise the same worker::run entry point used by the desktop UI, without a model.
cli <- logical(nrow(d))
for(i in seq_len(nrow(d))) {
 inp <- file.path(out,paste0('cli-',i,'-input.txt')); res <- file.path(out,paste0('cli-',i,'-output.txt'))
 writeBin(charToRaw(enc2utf8(d$source[i])),inp)
 stopifnot(identical(utf8ToInt(rawToChar(readBin(inp,'raw',n=file.info(inp)$size))),utf8ToInt(d$source[i])))
 code <- system2(exe,c('--translate',d$direction[i]),stdin=inp,stdout=res,stderr=file.path(out,paste0('cli-',i,'-stderr.txt')),timeout=10)
 actual <- rawToChar(readBin(res,'raw',n=file.info(res)$size));Encoding(actual)<-'UTF-8'
 cli[i] <- code==0 && identical(utf8ToInt(actual),utf8ToInt(d$target[i]))
}
write.csv(data.frame(direction=d$direction,passed=cli),file.path(out,'r-ui-path-checks.csv'),row.names=FALSE)
cat(sum(cli),'/',length(cli),'desktop-entry-path checks passed\n')
stopifnot(all(cli))
