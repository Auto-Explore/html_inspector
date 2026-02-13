# html/webappapis/structured-clone/structured-clone-detached-window-crash.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/structured-clone/structured-clone-detached-window-crash.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<title>window.structuredClone() doesn't crash when window is detached</title>
<link rel="help" href="https://crbug.com/1445375">
<body>
<script>
let i = document.createElement("iframe");
document.body.appendChild(i);
let i_win = i.contentWindow;
i.remove();
i_win.structuredClone("some data");
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
  "source_name": "html/webappapis/structured-clone/structured-clone-detached-window-crash.html"
}
```
