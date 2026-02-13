# html/rendering/replaced-elements/the-option-element/option-with-br.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/rendering/replaced-elements/the-option-element/option-with-br.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>option element with br child</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#concept-option-label">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#dom-option-text">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-select-element-2">

<link rel="match" href="option-with-br-ref.html">

<p>This test passes if the option element displays three options:</p>

<pre>a
b
ab</pre>

<p>Importantly the third option must not be split across two lines.</p>

<select multiple>
  <option>a</option>
  <option>b</option>
  <option id="manipulate-me"></option>
</select>

<script>
"use strict";
const option = document.querySelector("#manipulate-me");

option.appendChild(document.createTextNode("a"));
option.appendChild(document.createElement("br"));
option.appendChild(document.createTextNode("b"));
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.option.empty_without_label",
      "message": "Element “option” without attribute “label” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 721,
        "byte_start": 712,
        "col": 30,
        "line": 22
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
  "source_name": "html/rendering/replaced-elements/the-option-element/option-with-br.html"
}
```
