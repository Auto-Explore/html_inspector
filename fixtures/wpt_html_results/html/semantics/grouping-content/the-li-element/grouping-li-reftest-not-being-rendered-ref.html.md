# html/semantics/grouping-content/the-li-element/grouping-li-reftest-not-being-rendered-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-not-being-rendered-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>List items that are not being rendered do not participate in numbering</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<p>This test matches if the list displays similar to the following</p>

<pre>1. A
2. B
3. D
4. E
5. G</pre>

<hr>

<ol>
  <li value="1">A</li>
  <li value="2">B</li>
  <li value="3">D</li>
  <li value="4">E</li>
  <li value="5">G</li>
</ol>
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-not-being-rendered-ref.html"
}
```
