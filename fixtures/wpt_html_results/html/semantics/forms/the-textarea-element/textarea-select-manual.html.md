# html/semantics/forms/the-textarea-element/textarea-select-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/textarea-select-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>HTMLTextAreaElement Test: select()</title>
<link rel="author" title="Intel" href="http://www.intel.com/">
<meta name="flags" content="interact">

<p>Test passes if content of the input area is selected</p>

<textarea id="test_obj">1234567</textarea>
<script>
var textarea = document.querySelector("#test_obj");
textarea.select();
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
  "source_name": "html/semantics/forms/the-textarea-element/textarea-select-manual.html"
}
```
