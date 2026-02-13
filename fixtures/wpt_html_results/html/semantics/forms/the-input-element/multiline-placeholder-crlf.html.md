# html/semantics/forms/the-input-element/multiline-placeholder-crlf.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/multiline-placeholder-crlf.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html class="reftest-wait">
<meta charset="utf-8">
<title>input multiline placeholder (CRLF)</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#the-placeholder-attribute">
<meta name="assert" content="input element's placeholder strips newlines (CRLF)">
<link rel="match" href="multiline-placeholder-ref.html">
<input placeholder="this is
a multiline

placeholder">
<input placeholder="this is&#xd;&#xa;a multiline&#xd;&#xa;&#xd;&#xa;placeholder">
<input id="dynamic">
<script>
  document.querySelector("#dynamic")
          .setAttribute("placeholder", "this is\r\na multiline\r\n\r\nplaceholder");
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
        "byte_end": 421,
        "byte_start": 364,
        "col": 1,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.placeholder.linebreak",
      "message": "Bad value “this is\r\na multiline\r\n\r\nplaceholder” for attribute “placeholder” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 504,
        "byte_start": 423,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 431,
        "byte_start": 430,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 452,
        "byte_start": 451,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 462,
        "byte_start": 461,
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
  "source_name": "html/semantics/forms/the-input-element/multiline-placeholder-crlf.html"
}
```
