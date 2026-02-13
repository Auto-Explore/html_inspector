# html/semantics/interactive-elements/contextmenu-historical.html

Counts:
- errors: 0
- warnings: 17
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interactive-elements/contextmenu-historical.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>menu element removed properties</title>
<link rel="help" href="https://html.spec.whatwg.org/multipage/#the-menu-element">
<link rel="help" href="https://github.com/whatwg/html/pull/2742">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<menu type="context" label="label">
  <menuitem>Text</menuitem>
  <menuitem type="checkbox" checked>Checked</menuitem>
  <menuitem disabled>Disabled</menuitem>
  <menuitem default>Default</menuitem>
</menu>

<script>
"use strict";

const menu = document.querySelector("menu");
const menuitem = document.querySelector("menuitem");

test(() => {
  assert_false("HTMLMenuItemElement" in window, "the HTMLMenuItemElement interface must not exist");
  assert_equals(menuitem.constructor, HTMLUnknownElement, "A <menuitem> must be HTMLUnknownElement");

  for (const prop of ["type", "label", "icon", "disabled", "checked", "radiogroup", "default"]) {
    assert_false(prop in menuitem, `menuitem.${prop} must not be present`);
  }
}, "HTMLMenuItemElement must not be not present");

test(() => {
  const potentialBadLocations = [
    window,
    document,
    HTMLElement.prototype,
    SVGElement.prototype,
    Document.prototype,
    HTMLDocument.prototype,
    Element.prototype
  ];
  for (const location of potentialBadLocations) {
    assert_false("onshow" in location,
      `${location.constructor.name} must not have a property "onshow"`);
  }
}, `onshow must not be present on the GlobalEventHandlers locations`);

test(() => {
  assert_false("RelatedEvent" in window);
}, "RelatedEvent must not be present");

test(() => {
  assert_false("contextMenu" in HTMLElement.prototype,
    "HTMLElement's prototype must not have a property \"contextMenu\"");
  assert_false("contextMenu" in document.createElement("div"),
    "A div must not have a property \"contextMenu\"");
}, "el.contextMenu must not be present");

test(() => {
  assert_false("type" in menu);

  menu.type = "toolbar";
  assert_equals(menu.getAttribute("type"), "context");
}, "menu.type must not exist or reflect the content attribute");

test(() => {
  assert_false("label" in menu);

  menu.label = "new label";
  assert_equals(menu.getAttribute("label"), "label");
}, "menu.label must not exist or reflect the content attribute");

test(() => {
  assert_array_equals(document.querySelectorAll("menuitem:enabled"), []);
}, ":enabled must not match menuitems");

test(() => {
  assert_array_equals(document.querySelectorAll("menuitem:disabled"), []);
}, ":disabled must not match menuitems");

test(() => {
  assert_array_equals(document.querySelectorAll("menuitem:checked"), []);
}, ":checked must not match menuitems");

test(() => {
  try {
    assert_array_equals(document.querySelectorAll("menuitem:default"), []);
  } catch (e) {
    // Not everyone has implemented :default as of the time of this writing.
    if (e.name !== "SyntaxError") {
      throw e;
    }
  }
}, ":default must not match menuitems");

test(() => {
  assert_equals(getComputedStyle(menu).display, "block");
}, "The user-agent stylesheet must leave type=\"context\" menus as block display like other menus");

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 389,
        "byte_start": 379,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 389,
        "byte_start": 379,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 389,
        "byte_start": 379,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 389,
        "byte_start": 379,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 441,
        "byte_start": 407,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 441,
        "byte_start": 407,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 441,
        "byte_start": 407,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 441,
        "byte_start": 407,
        "col": 3,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 481,
        "byte_start": 462,
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
        "byte_end": 481,
        "byte_start": 462,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 481,
        "byte_start": 462,
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
        "byte_end": 481,
        "byte_start": 462,
        "col": 3,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 521,
        "byte_start": 503,
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
        "byte_end": 521,
        "byte_start": 503,
        "col": 3,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.menu.child.not_allowed",
      "message": "Element “menuitem” not allowed as child of “menu” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 521,
        "byte_start": 503,
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
        "byte_end": 521,
        "byte_start": 503,
        "col": 3,
        "line": 13
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
  "source_name": "html/semantics/interactive-elements/contextmenu-historical.html"
}
```
