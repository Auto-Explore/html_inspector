# html/semantics/menu/tentative/button-invoke-menulist.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/button-invoke-menulist.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<button id=btn commandfor="m">Actions</button>

<menulist id=m>
 <menuitem>Action 1</menuitem>
 <menuitem>Action 2</menuitem>
</menulist>

<script>
const btn = document.getElementById("btn");
const menulist = document.querySelector("menulist");

test(() => {
 btn.setAttribute("command", "toggle-popover");
 btn.disabled = true;
 btn.click();
 assert_false(menulist.matches(':popover-open'),
    'The menulist should not open because the button is disabled.');

 btn.disabled = false;
 btn.click();
 assert_true(menulist.matches(':popover-open'),
    'The menulist should be able to open successfully.');
 menulist.hidePopover();
}, "Button with command=toggle-popover can invoke menulist popover.");

test(() => {
 btn.setAttribute("command", "toggle-menu");
 btn.disabled = true;
 btn.click();
 assert_false(menulist.matches(':popover-open'),
    'The menulist should not open because the button is disabled.');

 btn.disabled = false;
 btn.click();
 assert_true(menulist.matches(':popover-open'),
    'The menulist should be able to open successfully.');
 menulist.hidePopover();
}, "Button with command=toggle-menu can invoke menulist popover.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 186,
        "byte_start": 171,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 186,
        "byte_start": 171,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 198,
        "byte_start": 188,
        "col": 2,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 198,
        "byte_start": 188,
        "col": 2,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 229,
        "byte_start": 219,
        "col": 2,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 229,
        "byte_start": 219,
        "col": 2,
        "line": 9
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
  "source_name": "html/semantics/menu/tentative/button-invoke-menulist.html"
}
```
