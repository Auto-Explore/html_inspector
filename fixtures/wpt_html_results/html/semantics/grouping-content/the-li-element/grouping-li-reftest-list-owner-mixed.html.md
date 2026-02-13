# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-mixed.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-mixed.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>list owner is calculated to be nearest ancestor ul or ul (but not dir) if it exists</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#ordinal-value">
<link rel="help" href="https://html.spec.whatwg.org/multipage/semantics.html#list-owner">

<link rel="match" href="grouping-li-reftest-list-owner-mixed-ref.html">

<style>
  li {
    list-style-type: decimal;
  }

  .list-item {
    display: list-item;
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
          3. K
          4. L</pre>

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
    <ol>
      <li>F</li>
      <span class="list-item">G</span>
    </ol>
  </div>
  <li>H</li>
  <ol>
    <li>I</li>
    <li>
      J
      <dir>
        <li>K</li>
        <li>L</li>
      </dir>
    </li>
  </ol>
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
        "byte_end": 838,
        "byte_start": 834,
        "col": 5,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 866,
        "byte_start": 862,
        "col": 7,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “span” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 883,
        "byte_start": 879,
        "col": 7,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1049,
        "byte_start": 1044,
        "col": 7,
        "line": 58
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1062,
        "byte_start": 1058,
        "col": 9,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1081,
        "byte_start": 1077,
        "col": 9,
        "line": 60
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-mixed.html"
}
```
