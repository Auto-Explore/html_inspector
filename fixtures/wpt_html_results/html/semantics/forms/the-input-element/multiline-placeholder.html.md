# html/semantics/forms/the-input-element/multiline-placeholder.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/multiline-placeholder.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<meta charset="utf-8">
<title>input multiline placeholder</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#the-placeholder-attribute">
<meta name="assert" content="input element's placeholder strips newlines">
<link rel="match" href="multiline-placeholder-ref.html">
<input placeholder="this is
a multiline

placeholder">
<input placeholder="this is&#xa;a multiline&#xa;&#xa;placeholder">
<input id="dynamic">
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
      "category": "Html",
      "code": "html.placeholder.linebreak",
      "message": "Bad value “this is\na multiline\n\nplaceholder” for attribute “placeholder” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 397,
        "byte_start": 343,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.placeholder.linebreak",
      "message": "Bad value “this is\na multiline\n\nplaceholder” for attribute “placeholder” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 464,
        "byte_start": 398,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/forms/the-input-element/multiline-placeholder.html"
}
```
