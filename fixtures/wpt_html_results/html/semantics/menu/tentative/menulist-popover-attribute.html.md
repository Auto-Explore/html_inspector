# html/semantics/menu/tentative/menulist-popover-attribute.html

Counts:
- errors: 0
- warnings: 8
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/menulist-popover-attribute.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<link rel=author href=mailto:dizhangg@chromium.org>
<link rel=help href=https://open-ui.org/components/menu.explainer>

<menulist id="menu">
 <menuitem>Command 1</menuitem>
 <menuitem>Command 2</menuitem>
</menulist>

<script>
test(() => {
  assert_equals(menu.popover, null);
  menu.showPopover();
  assert_true(menu.matches(':popover-open'));
  menu.hidePopover();

  menu.popover = 'auto';
  assert_equals(menu.popover, 'auto');
  menu.showPopover();
  assert_true(menu.matches(':popover-open'));
  menu.hidePopover();

  menu.popover = 'manual';
  assert_equals(menu.popover, 'manual');
  menu.showPopover();
  assert_true(menu.matches(':popover-open'));
  menu.hidePopover();

  menu.popover = null;
  assert_equals(menu.popover, null);
  menu.showPopover();
  assert_true(menu.matches(':popover-open'));
  menu.hidePopover();
}, "menulist is a popover by default.");
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
        "byte_end": 262,
        "byte_start": 242,
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
        "byte_end": 262,
        "byte_start": 242,
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
        "byte_end": 274,
        "byte_start": 264,
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
        "byte_end": 274,
        "byte_start": 264,
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
        "byte_end": 306,
        "byte_start": 296,
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
        "byte_end": 306,
        "byte_start": 296,
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
  "source_name": "html/semantics/menu/tentative/menulist-popover-attribute.html"
}
```
