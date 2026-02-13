# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-ol.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-ol.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>list owner is calculated to be narest ancestor ol if it exists</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<link rel="match" href="grouping-li-reftest-list-owner-ol-ref.html">

<p>This test matches if the list displays similar to the following</p>

<pre>1. A
2. B
3. C
4. D
5. E
     1. F
     2. G
6. H
     1. I
     2. J
          1. K
          2. L</pre>

<hr>

<ol>
  <li>A</li>
  <li>B</li>
  <div>
    <li>C</li>
    <span>
      <li>D</li>
      <li>E</li>
    </span>
    <ol>
      <li>F</li>
      <li>G</li>
    </ol>
  </div>
  <li>H</li>
  <ol>
    <li>I</li>
    <li>
      J
      <ol>
        <li>K</li>
        <li>L</li>
      </ol>
    </li>
  </ol>
</ol>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 680,
        "byte_start": 676,
        "col": 5,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 708,
        "byte_start": 704,
        "col": 7,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 725,
        "byte_start": 721,
        "col": 7,
        "line": 34
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-ol.html"
}
```
