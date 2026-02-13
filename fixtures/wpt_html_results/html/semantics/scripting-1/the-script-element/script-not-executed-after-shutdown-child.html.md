# html/semantics/scripting-1/the-script-element/script-not-executed-after-shutdown-child.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/script-not-executed-after-shutdown-child.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>Script is not executed after script thread is shutdown</title>
<script>
onload = function() {
  script_executed = parent.script_executed;
  s = document.createElement('script');
  s.type = 'text/javascript';
  s.src = 'script-not-executed-after-shutdown.js?pipe=trickle(d3)';
  document.body.appendChild(s);
};
</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/script-not-executed-after-shutdown-child.html"
}
```
