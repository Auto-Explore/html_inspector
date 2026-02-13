# html/browsers/browsing-the-web/unloading-documents/prompt/manual-005.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/unloading-documents/prompt/manual-005.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>Event loop pause for beforeunload</title>
<script>
var counter = 0;

onload = function count() {
  document.getElementById("log").textContent = counter++
  setTimeout(count, 200);
}

addEventListener("beforeunload",
function(e) {
  e.preventDefault()
},
false);
</script>
<ul>
<li>Click on the link below. When the prompt appears the counter at the bottom must stop incrementing.
<li>Opt not to leave the page. The counter must start incrementing again
</ul>
<p><a href="">Click here</a>
<div id="log"></div>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/browsing-the-web/unloading-documents/prompt/manual-005.html"
}
```
