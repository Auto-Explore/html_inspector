# html/dom/elements/global-attributes/style-01.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/style-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>The style attribute</title>
<link rel="match" href="style-01-ref.html">
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-style-attribute">
<link rel="help" href="http://www.w3.org/TR/css-style-attr/#syntax">
<link rel="help" href="http://www.w3.org/TR/CSS21/cascade.html#cascading-order">
<link rel="help" href="http://www.w3.org/TR/CSS21/cascade.html#specificity">
<style>
#idsel { background: red; }
#idsel2 { background: limegreen !important; }
</style>
<div id="test">
<p style="background:limegreen">This line should have a green background.
<p style="/**/background:limegreen">This line should have a green background.
<p style="background/**/:limegreen">This line should have a green background.
<p style="background:/**/limegreen">This line should have a green background.
<p style="background:limegreen/**/">This line should have a green background.
<p id="idsel1" style="background:limegreen">This line should have a green background.
<p id="idsel2" style="background:red">This line should have a green background.
<p style="background:limegreen; background:r/**/ed">This line should have a green background.
<p style="background:limegreen;}">This line should have a green background.
<p style="};background:limegreen">This line should have a green background.
<p style="background:red;};background:limegreen">This line should have a green background.
<p style="background:limegreen;{background:red}">This line should have a green background.
</div>
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
  "source_name": "html/dom/elements/global-attributes/style-01.html"
}
```
