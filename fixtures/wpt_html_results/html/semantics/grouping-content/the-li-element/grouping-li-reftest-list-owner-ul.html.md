# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-ul.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-ul.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>list owner is calculated to be nearest ancestor ul if it exists</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<link rel="match" href="grouping-li-reftest-list-owner-ul-ref.html">

<style>
  li {
    list-style-type: decimal;
  }
</style>

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

<ul>
  <li>A</li>
  <li>B</li>
  <div>
    <li>C</li>
    <span>
      <li>D</li>
      <li>E</li>
    </span>
    <ul>
      <li>F</li>
      <li>G</li>
    </ul>
  </div>
  <li>H</li>
  <ul>
    <li>I</li>
    <li>
      J
      <ul>
        <li>K</li>
        <li>L</li>
      </ul>
    </li>
  </ul>
</ul>
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
        "byte_end": 740,
        "byte_start": 736,
        "col": 5,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 768,
        "byte_start": 764,
        "col": 7,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 785,
        "byte_start": 781,
        "col": 7,
        "line": 40
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-ul.html"
}
```
