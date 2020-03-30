# Simple RSS Reader and Translator

Everything is currently hard-coded, so this project is likely of little
use to anyone but me.

##

Overview:

 - Downloads RSS from a news website
 - Grabs `title` and `description` in Italian
 - Uses translate.yandex.net v1.5 api to translate to English
 - Displays both versions, original link, and translation link, then exits

The included `daily_news_rss.sh` is executed from a cronjob each afternoon,
emailing the output from the above script to me daily.
