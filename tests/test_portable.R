# Development-only R tests of extracted native app. No R/Python app dependency.
args <- commandArgs(trailingOnly=TRUE)
stopifnot(length(args)>=2L)
pack <- normalizePath(args[1],winslash='/',mustWork=TRUE)
report <- normalizePath(args[2],winslash='/',mustWork=TRUE)
exe <- file.path(pack,'lugha-desk.exe')
stopifnot(file.exists(exe))
original_path <- Sys.getenv('PATH')
Sys.setenv(PATH='')
stopifnot(Sys.which('python')=='')
# R can locate its own Rscript through R_HOME even with PATH empty; this is
# the test harness, not evidence of a translator runtime dependency.
saveRDS(list(PATH='',python_on_PATH=unname(Sys.which('python')),Rscript_on_PATH=unname(Sys.which('Rscript'))),file.path(report,'isolated-path-test.rds'))
# Greeting fixtures are unvalidated drafts, not gold references or quality ratings.
langs <- data.frame(code=c('sw','so','lg','ha','yo','ig','wo','zu','xh','st','tn','af'),
 language=c('Swahili','Somali','Luganda','Hausa','Yoruba','Igbo','Wolof','isiZulu','isiXhosa','Sesotho','Setswana','Afrikaans'),
 region=c(rep('East Africa',3),rep('West Africa',4),rep('South Africa',5)),
 reverse=c('Habari za asubuhi.','Subax wanaagsan.','Wasuze otya?','Sannu.','Ẹ káàárọ̀.','Ụtụtụ ọma.','Na nga def?','Sawubona.','Molo.','Lumela.','Dumela.','Goeiemôre.'),stringsAsFactors=FALSE)
cases <- do.call(rbind,lapply(seq_len(nrow(langs)),function(i)data.frame(
 direction=c(paste0('en-',langs$code[i]),paste0(langs$code[i],'-en')),
 language=langs$language[i],region=langs$region[i],source=c('Good morning.',langs$reverse[i]),kind='greeting smoke',stringsAsFactors=FALSE)))
cases <- rbind(cases,data.frame(direction=c('en-sw','sw-en','en-sw'),language='Swahili',region='East Africa',
 source=c('He borrowed my bicycle.','Aliazima baiskeli yangu.','Émile gave Mary the book.'),
 kind=c('known meaning risk','known meaning risk','Unicode/name risk'),stringsAsFactors=FALSE))
rows <- list()
for(i in seq_len(nrow(cases))){
 input <- tempfile();output <- tempfile();error <- tempfile()
 writeLines(enc2utf8(cases$source[i]),input,useBytes=TRUE)
 cat('Testing',cases$direction[i],cases$kind[i],'\n');flush.console()
 start <- proc.time()[['elapsed']]
 status <- suppressWarnings(system2(exe,c('--translate',cases$direction[i]),stdin=input,stdout=output,stderr=error,timeout=340))
 result <- paste(readLines(output,encoding='UTF-8',warn=FALSE),collapse='\n')
 errors <- paste(readLines(error,encoding='UTF-8',warn=FALSE),collapse='\n')
 passed <- identical(as.integer(status),0L)&&nchar(trimws(result))>0&&!nzchar(errors)
 rows[[length(rows)+1L]] <- data.frame(cases[i,],output=result,seconds=proc.time()[['elapsed']]-start,
  technical_result=if(passed)'passed' else 'failed',error=errors,
  meaning_review='not independently reviewed; not a quality pass',stringsAsFactors=FALSE)
 write.csv(do.call(rbind,rows),file.path(report,'portable-r-results.csv'),row.names=FALSE,fileEncoding='UTF-8')
 unlink(c(input,output,error))
 if(i==1L&&!passed)stop('Initial native inference failed; inspect portable-r-results.csv before broader tests.')
}
Sys.setenv(PATH=original_path)
capture.output(sessionInfo(),file=file.path(report,'portable-r-session.txt'))
results <- do.call(rbind,rows);print(results[,c('direction','language','output','seconds','technical_result')])
cat('Technical passes:',sum(results$technical_result=='passed'),'/',nrow(results),'\n')
cat('All meaning judgments remain unverified. Model listings and greetings do not establish general quality.\n')
if(any(results$technical_result!='passed'))quit(status=1L)
