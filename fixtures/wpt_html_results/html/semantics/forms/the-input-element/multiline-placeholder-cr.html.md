# html/semantics/forms/the-input-element/multiline-placeholder-cr.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-input-element/multiline-placeholder-cr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html><html class="reftest-wait"><meta charset="utf-8"><title>input multiline placeholder (CR)</title><link rel="help" href="https://html.spec.whatwg.org/multipage/input.html#the-placeholder-attribute"><meta name="assert" content="input element's placeholder strips newlines (CR)"><link rel="match" href="multiline-placeholder-ref.html"><input placeholder="this isa multilineplaceholder"><input placeholder="this is&#xd;a multiline&#xd;&#xd;placeholder"><input id="dynamic"><script>  document.querySelector("#dynamic")          .setAttribute("placeholder", "this is\ra multiline\r\rplaceholder");  document.documentElement.classList.remove("reftest-wait");</script></html>
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
        "byte_end": 407,
        "byte_start": 353,
        "col": 354,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.placeholder.linebreak",
      "message": "Bad value “this is\ra multiline\r\rplaceholder” for attribute “placeholder” on element “input”.",
      "severity": "Warning",
      "span": {
        "byte_end": 474,
        "byte_start": 408,
        "col": 409,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 416,
        "byte_start": 415,
        "col": 409,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 432,
        "byte_start": 431,
        "col": 409,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 437,
        "byte_start": 436,
        "col": 409,
        "line": 1
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
  "source_name": "html/semantics/forms/the-input-element/multiline-placeholder-cr.html"
}
```
