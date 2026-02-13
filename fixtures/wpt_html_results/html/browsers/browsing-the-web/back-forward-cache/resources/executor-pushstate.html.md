# html/browsers/browsing-the-web/back-forward-cache/resources/executor-pushstate.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/browsing-the-web/back-forward-cache/resources/executor-pushstate.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE HTML>
<script src="/common/dispatcher/dispatcher.js"></script>
<script src="event-recorder.js" type="module"></script>
<script src="worker-helper.js" type="module"></script>
<script type="module">
// This is mostly the same as `executor.html`, except for
// `isLoadedFromPushState` is set here, in order to detect whether the page
// was loaded from `executor.html` or `executor-pushstate.html`.
// Full executor functionality is still needed to handle remote script
// execution requests etc.
window.isLoadedFromPushState = true;
</script>
<script src="executor.js" type="module"></script>
<script src="disable_bfcache.js" type="module"></script>
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
  "source_name": "html/browsers/browsing-the-web/back-forward-cache/resources/executor-pushstate.html"
}
```
