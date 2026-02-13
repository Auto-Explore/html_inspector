# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-mixed-ref.html

Counts:
- errors: 0
- warnings: 6
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-mixed-ref.html",
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
          3. K
          4. L</pre>

<hr>

<ol>
  <li value="1">A</li>
  <li value="2">B</li>
  <li value="3">C</li>
  <li value="4">D</li>
  <li value="5">E</li>
  <ol>
    <li value="1">F</li>
    <li value="2">G</li>
  </ol>
  <li value="6">H</li>
  <ol>
    <li value="1">I</li>
    <li value="2">
      J
      <dir>
        <li value="3">K</li>
        <li value="4">L</li>
      </dir>
    </li>
  </ol>
</ol>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 921,
        "byte_start": 916,
        "col": 7,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 944,
        "byte_start": 930,
        "col": 9,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 944,
        "byte_start": 930,
        "col": 9,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 973,
        "byte_start": 959,
        "col": 9,
        "line": 48
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 973,
        "byte_start": 959,
        "col": 9,
        "line": 48
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-mixed-ref.html"
}
```
