# html/semantics/forms/the-textarea-element/multiline-placeholder.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/multiline-placeholder.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<meta charset="utf-8">
<title>textarea multiline placeholder</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/form-elements.html#attr-textarea-placeholder">
<meta name="assert" content="textarea element's placeholder preserves newlines">
<link rel="match" href="multiline-placeholder-ref.html">
<link rel="stylesheet" href="support/placeholder.css">
<textarea rows="5" placeholder="this is
a multiline

placeholder"></textarea>
<textarea rows="5" placeholder="this is&#xa;a multiline&#xa;&#xa;placeholder"></textarea>
<textarea rows="5" id="dynamic"></textarea>
<script>
  document.querySelector("#dynamic")
          .setAttribute("placeholder", "this is\na multiline\n\nplaceholder");
  document.documentElement.classList.remove("reftest-wait");
</script>
</html>


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
  "source_name": "html/semantics/forms/the-textarea-element/multiline-placeholder.html"
}
```
