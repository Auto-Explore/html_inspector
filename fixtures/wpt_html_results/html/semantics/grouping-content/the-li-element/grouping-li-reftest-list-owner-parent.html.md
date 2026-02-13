# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-parent.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-parent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>list owner is calculated to be the parent if there is no ancestor ul/ol/menu</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<link rel="match" href="grouping-li-reftest-list-owner-parent-ref.html">

<style>
  li {
    list-style-type: decimal;
    list-style-position: inside;
  }

  .container {
    padding: 50px;
  }
</style>

<p>This test matches if the list displays similar to the following</p>

<pre>1. A
2. B
1. C
1. D
       1. E
3. F</pre>

<hr>

<div class="container">
  <li>A</li>
  <li>B</li>
  <div>
    <li>C</li>
    <span>
      <li>D</li>
    </span>
  </div>
  <blockquote>
    <li>E</li>
  </blockquote>
  <li>F</li>
</div>
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
        "byte_end": 749,
        "byte_start": 745,
        "col": 3,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 762,
        "byte_start": 758,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 785,
        "byte_start": 781,
        "col": 5,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 813,
        "byte_start": 809,
        "col": 7,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “blockquote” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 864,
        "byte_start": 860,
        "col": 5,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 893,
        "byte_start": 889,
        "col": 3,
        "line": 44
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-parent.html"
}
```
