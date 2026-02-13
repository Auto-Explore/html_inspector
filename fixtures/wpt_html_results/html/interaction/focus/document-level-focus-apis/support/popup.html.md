# html/interaction/focus/document-level-focus-apis/support/popup.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/interaction/focus/document-level-focus-apis/support/popup.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
Popup <input id="input">
<script>
window.onload = async () => {
  try {
    if (!document.hasFocus()) {
      const input = document.getElementById("input");
      input.focus();
      await new Promise(r => input.onfocus = r);
    }
    opener.postMessage(`focus = ${document.hasFocus()}`, "*");
  } catch(e) {
    opener.postMessage(`${e.name}: $(e.message)`, "*");
  }
};
</script>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 431,
        "byte_start": 424,
        "col": 1,
        "line": 18
      }
    },
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
  "source_name": "html/interaction/focus/document-level-focus-apis/support/popup.html"
}
```
