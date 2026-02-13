# html/semantics/menu/tentative/menu-elements-default-style.html

Counts:
- errors: 0
- warnings: 18
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/menu-elements-default-style.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8" />
<link rel=author href=mailto:dom@chromium.org>
<link rel=help href=https://open-ui.org/components/menu.explainer>
<link rel="match" href="menu-elements-default-style-ref.html">

<style>
body { background-color: orange; }
</style>

<menubar>
  <menuitem>Menubar item A</menuitem>
  <menuitem>Menubar item B</menuitem>
  <menuitem>Menubar item C</menuitem>
</menubar>

<br>

<menulist>
  <menuitem>Menulist item A</menuitem>
  <menuitem>Menulist item B</menuitem>
  <hr>
  <menuitem>Menulist item C</menuitem>
</menulist>

<script>
const menulist = document.querySelector("menulist");
menulist.showPopover();
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menubar” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 281,
        "byte_start": 272,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menubar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 281,
        "byte_start": 272,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 294,
        "byte_start": 284,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 294,
        "byte_start": 284,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 332,
        "byte_start": 322,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 332,
        "byte_start": 322,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 370,
        "byte_start": 360,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 370,
        "byte_start": 360,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 424,
        "byte_start": 414,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 424,
        "byte_start": 414,
        "col": 1,
        "line": 19
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 437,
        "byte_start": 427,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 437,
        "byte_start": 427,
        "col": 3,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 476,
        "byte_start": 466,
        "col": 3,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 476,
        "byte_start": 466,
        "col": 3,
        "line": 21
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 522,
        "byte_start": 512,
        "col": 3,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 522,
        "byte_start": 512,
        "col": 3,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/menu/tentative/menu-elements-default-style.html"
}
```
