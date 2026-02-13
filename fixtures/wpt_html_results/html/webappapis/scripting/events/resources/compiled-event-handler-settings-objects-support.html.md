# html/webappapis/scripting/events/resources/compiled-event-handler-settings-objects-support.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/resources/compiled-event-handler-settings-objects-support.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>This will be in an iframe</title>

<script>
window.name = "iframe";
</script>

<body onbeforeunload="return { toString: parent.postMessage.bind(parent, 'PASS', '*') };">

<!-- window.open() uses the entry settings object to determine how the URL will be parsed -->
<button onclick="var w = window.open('open-window.html'); w.onload = () => { parent.onWindowLoaded(w.document.URL); };">This will be clicked</button>
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
  "source_name": "html/webappapis/scripting/events/resources/compiled-event-handler-settings-objects-support.html"
}
```
