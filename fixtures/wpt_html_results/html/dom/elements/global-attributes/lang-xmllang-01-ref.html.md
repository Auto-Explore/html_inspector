# html/dom/elements/global-attributes/lang-xmllang-01-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/dom/elements/global-attributes/lang-xmllang-01-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<title>Languages</title>
<link rel="author" title="Ms2ger" href="mailto:ms2ger@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-lang-and-xml:lang-attributes">
<link rel="help" href="http://www.w3.org/TR/CSS2/selector.html#lang">
<meta name="flags" content="css21">
<style>
#test > * { background: limegreen; }
</style>
<body>
<p>All lines below should have a green background.</p>
<div id="test">
<div><p>{}{lang}{en}</p></div>
<div><p>{}{xml:lang}{en}</p></div>
<div><div><p>Parent: {}{lang}{en}</p></div></div>
<div><div><p>Parent: {}{xml:lang}{en}</p></div></div>
<div><p>{xml}{lang}{en}</p></div>
<div><p>{xml}{lang}{en} - {lang}{de}</p></div>
<div><p>{xml}{lang}{de} - {lang}{en}</p></div>
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
  "source_name": "html/dom/elements/global-attributes/lang-xmllang-01-ref.html"
}
```
