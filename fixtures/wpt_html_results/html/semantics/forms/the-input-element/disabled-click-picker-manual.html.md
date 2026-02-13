# html/semantics/forms/the-input-element/disabled-click-picker-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/disabled-click-picker-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Disabled input elements must not open pickers</title>
<p>
  Click the Open buttons below. If clicking them does not open any pickers, then consider this as a passing test.<br>
  (This is manual because we don't have an event to check whether the picker is opened or not.)
</p>
<input disabled type="color" id="color"><button>Open</button><br>
<input disabled type="file" id="file"><button>Open</button>
<script>
  for (const button of document.getElementsByTagName("button")) {
    button.onclick = () => {
      const input = button.previousElementSibling;
      input.dispatchEvent(new MouseEvent("click"));
    }
  }
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
  "source_name": "html/semantics/forms/the-input-element/disabled-click-picker-manual.html"
}
```
