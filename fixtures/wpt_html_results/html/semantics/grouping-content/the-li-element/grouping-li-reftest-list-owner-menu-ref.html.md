# html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-menu-ref.html

Counts:
- errors: 0
- warnings: 15
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-menu-ref.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>list owner is calculated to be narest ancestor menu if it exists</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">
<link rel="author" title="Peng Zhou" href="mailto:zhoupeng.1996@bytedance.com">
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
          1. K
          2. L</pre>

<hr>

<menu>
  <li value="1">A</li>
  <li value="2">B</li>
  <li value="3">C</li>
  <li value="4">D</li>
  <li value="5">E</li>
  <menu>
    <li value="1">F</li>
    <li value="2">G</li>
  </menu>
  <li value="6">H</li>
  <menu>
    <li value="1">I</li>
    <li value="2">
      J
      <menu>
        <li value="1">K</li>
        <li value="2">L</li>
      </menu>
    </li>
  </menu>
</menu>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 727,
        "byte_start": 713,
        "col": 3,
        "line": 33
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 750,
        "byte_start": 736,
        "col": 3,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 773,
        "byte_start": 759,
        "col": 3,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 796,
        "byte_start": 782,
        "col": 3,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 819,
        "byte_start": 805,
        "col": 3,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 834,
        "byte_start": 828,
        "col": 3,
        "line": 38
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 853,
        "byte_start": 839,
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
        "byte_end": 878,
        "byte_start": 864,
        "col": 5,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 911,
        "byte_start": 897,
        "col": 3,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.menu.not_allowed",
      "message": "Element “menu” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 926,
        "byte_start": 920,
        "col": 3,
        "line": 43
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 945,
        "byte_start": 931,
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
        "byte_end": 970,
        "byte_start": 956,
        "col": 5,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.li.value.disallowed",
      "message": "Attribute “value” not allowed on element “li” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 1014,
        "byte_start": 1000,
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
        "byte_end": 1043,
        "byte_start": 1029,
        "col": 9,
        "line": 49
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
  "source_name": "html/semantics/grouping-content/the-li-element/grouping-li-reftest-list-owner-menu-ref.html"
}
```
