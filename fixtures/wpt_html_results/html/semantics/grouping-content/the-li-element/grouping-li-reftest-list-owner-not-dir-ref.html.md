# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-not-dir-ref.html

Counts:
- errors: 0
- warnings: 16
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-not-dir-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>The dir element is not treated specially when calculating list owners</title>
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
     6. F
     7. G
8. H
     9. I
     10. J
          11. K
          12. L</pre>

<hr>

<ol>
  <li value="1">A</li>
  <li value="2">B</li>
  <li value="3">C</li>
  <li value="4">D</li>
  <li value="5">E</li>
  <dir>
    <li value="6">F</li>
    <li value="7">G</li>
  </dir>
  <li value="8">H</li>
  <dir>
    <li value="9">I</li>
    <li value="10">
      J
      <dir>
        <li value="11">K</li>
        <li value="12">L</li>
      </dir>
    </li>
  </dir>
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
        "byte_end": 759,
        "byte_start": 754,
        "col": 3,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 778,
        "byte_start": 764,
        "col": 5,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 778,
        "byte_start": 764,
        "col": 5,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 803,
        "byte_start": 789,
        "col": 5,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 803,
        "byte_start": 789,
        "col": 5,
        "line": 39
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 849,
        "byte_start": 844,
        "col": 3,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 868,
        "byte_start": 854,
        "col": 5,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 868,
        "byte_start": 854,
        "col": 5,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “dir” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 894,
        "byte_start": 879,
        "col": 5,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 894,
        "byte_start": 879,
        "col": 5,
        "line": 44
      }
    },
    {
      "category": "Html",
      "code": "html.element.dir.obsolete",
      "message": "The “dir” element is obsolete. Use the “ul” element instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 914,
        "byte_start": 909,
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
        "byte_end": 938,
        "byte_start": 923,
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
        "byte_end": 938,
        "byte_start": 923,
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
        "byte_end": 968,
        "byte_start": 953,
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
        "byte_end": 968,
        "byte_start": 953,
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-not-dir-ref.html"
}
```
