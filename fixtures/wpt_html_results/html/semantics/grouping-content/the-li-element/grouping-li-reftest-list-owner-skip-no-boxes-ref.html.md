# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-skip-no-boxes-ref.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-skip-no-boxes-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>list owner calculation skips elements that do not generate layout boxes</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<p>This test matches if the list displays similar to the following</p>

<pre>1. A
2. B
3. C
4. D
5. E
6. F
7. G
8. H
9. I
10. J
     1. K
     2. L</pre>

<hr>

<ol>
  <li value="1">A</li>
  <li value="2">B</li>
  <li value="3">C</li>
  <li value="4">D</li>
  <li value="5">E</li>
  <li value="6">F</li>
  <li value="7">G</li>
  <li value="8">H</li>
  <li value="9">I</li>
  <li value="10">
    J
    <ol>
      <li value="1">K</li>
      <li value="2">L</li>
    </ol>
  </li>
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-skip-no-boxes-ref.html"
}
```
