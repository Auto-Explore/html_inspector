# html/browsers/history/the-history-interface/non-automated/traverse_the_history_unload_prompt_1-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/history/the-history-interface/non-automated/traverse_the_history_unload_prompt_1-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="history.js"></script>
<script>
  onbeforeunload = function() {opener.beforeunload_ran = true; return "Opt to stay on the page"};

  opener.pages.push(id);
  if (!opener.started) {
    onload = function() {
      setTimeout(function() {
        opener.started = true;
        history.back();
      }, 100);
    }
  }
</script>
<p>You should see/have seen a prompt asking if you want to leave the page.</p>
<p>Opt to stay on the page</p>
<button onclick="onbeforeunload = null; opener.start_test_wait(); document.getElementsByTagName('button')[0].disabled = true;">Click here</button>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/history/the-history-interface/non-automated/traverse_the_history_unload_prompt_1-1.html"
}
```
