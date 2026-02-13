# html/semantics/forms/the-textarea-element/multiline-placeholder-cr.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/forms/the-textarea-element/multiline-placeholder-cr.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html><html class="reftest-wait"><meta charset="utf-8"><title>textarea multiline placeholder (CR)</title><link rel="help" href="https://html.spec.whatwg.org/multipage/form-elements.html#attr-textarea-placeholder"><meta name="assert" content="textarea element's placeholder preserves newlines (CR)"><link rel="match" href="multiline-placeholder-ref.html"><link rel="stylesheet" href="support/placeholder.css"><textarea rows="5" placeholder="this isa multilineplaceholder"></textarea><textarea rows="5" placeholder="this is&#xd;a multiline&#xd;&#xd;placeholder"></textarea><textarea rows="5" id="dynamic"></textarea><script>  document.querySelector("#dynamic")          .setAttribute("placeholder", "this is\ra multiline\r\rplaceholder");  document.documentElement.classList.remove("reftest-wait");</script></html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 511,
        "byte_start": 510,
        "col": 504,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 527,
        "byte_start": 526,
        "col": 504,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.tokenizer.charref_cr",
      "message": "A numeric character reference expanded to carriage return.",
      "severity": "Warning",
      "span": {
        "byte_end": 532,
        "byte_start": 531,
        "col": 504,
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
  "source_name": "html/semantics/forms/the-textarea-element/multiline-placeholder-cr.html"
}
```
