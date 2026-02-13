# html/semantics/menu/tentative/menubar-invoke-menulist.html

Counts:
- errors: 0
- warnings: 20
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/menubar-invoke-menulist.html",
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

<menubar>
  <menuitem id="menubaritem">More commands</menuitem>
  <menuitem>Command 2</menuitem>
  <menuitem id="opencheckable" commandfor="checkable" command="show-menu">Open checkable menu</menuitem>
</menubar>

<menulist id="more">
 <menuitem id="menulistitem" disabled>Command 1</menuitem>
 <menuitem>Command 2</menuitem>
</menulist>

<menulist id="checkable">
  <fieldset checkable>
    <menuitem id="checkable-menuitem">Show menu</menuitem>
  </fieldset>
</menulist>

<script>
const menubar = document.querySelector("menubar");
const menubaritem = document.getElementById("menubaritem");
const menulist = document.querySelector("menulist");
const menulistitem = document.getElementById("menulistitem");
const checkableMenuitem = document.getElementById("checkable-menuitem");

test(() => {
 assert_equals(menubar.constructor, HTMLMenuBarElement);
 assert_equals(menubaritem.constructor, HTMLMenuItemElement);
 assert_false(menubaritem.disabled);
 menubaritem.disabled = true;
 assert_true(menubaritem.disabled);

 assert_equals(menulist.constructor, HTMLMenuListElement);
 assert_equals(menulistitem.constructor, HTMLMenuItemElement);
 assert_true(menulistitem.disabled);
 menulistitem.disabled = false;
 assert_false(menulistitem.disabled);
}, "Menu elements are HTML elements.");

test(() => {
 menubaritem.setAttribute("command", "toggle-popover");
 menubaritem.setAttribute("commandfor", "more");

 menubaritem.disabled = true;
 menubaritem.click();
 assert_false(menulist.matches(':popover-open'),
    'The menulist should not open because the menuitem is disabled.');

 menubaritem.disabled = false;
 menubaritem.click();
 assert_true(menulist.matches(':popover-open'),
    'toggle-popover opens the menulist');

 menubaritem.click();
 assert_false(menulist.matches(':popover-open'),
    "toggle-popover closes the menulist");

 menubaritem.setAttribute("command", "show-popover");
 menubaritem.click();
 assert_true(menulist.matches(':popover-open'),
    "show-popover shows the menulist");

 menubaritem.setAttribute("command", "hide-popover");
 menubaritem.click();
 assert_false(menulist.matches(':popover-open'),
    "hide-popover hides the menulist");
}, "Menuitem with toggle-popover, show-popover, hide-popover commands can invoke menulist popover.");

test(() => {
 menubaritem.setAttribute("command", "toggle-menu");
 menubaritem.setAttribute("commandfor", "more");

 menubaritem.disabled = true;
 menubaritem.click();
 assert_false(menulist.matches(':popover-open'),
    'The menulist should not open because the menuitem is disabled.');

 menubaritem.disabled = false;
 menubaritem.click();
 assert_true(menulist.matches(':popover-open'),
    'toggle-menu opens the menulist');

 menubaritem.click();
 assert_false(menulist.matches(':popover-open'),
    "toggle-menu closes the menulist");

 menubaritem.setAttribute("command", "show-menu");
 menubaritem.click();
 assert_true(menulist.matches(':popover-open'),
    "show-menu opens the menulist");

 menubaritem.setAttribute("command", "hide-menu");
 menubaritem.click();
 assert_false(menulist.matches(':popover-open'),
    "hide-menu hides the menulist");
}, "Menuitem with toggle-menu, show-menu, hide-menu commands can invoke menulist popover.");

test(() => {
 menubaritem.command = "show-menu";
 menubaritem.commandForElement = menulist;

 menubaritem.click();
 assert_true(menulist.matches(':popover-open'),
    'show-menu opens the menulist.');

 menulist.hidePopover();
 assert_false(menulist.matches(':popover-open'),
    'The global hidePopover() method hides the menulist');
}, "hidePopover() on menulist closes the popover.");

test(() => {
 menubaritem.setAttribute("command", "toggle-menu");
 menubaritem.setAttribute("commandfor", "dne");
 menubaritem.click();
 assert_false(menulist.matches(':popover-open'),
    'The menulist should not open because the menuitem commandfor is invalid');

 menubaritem.setAttribute("command", "toggle-menu-dne");
 menubaritem.setAttribute("commandfor", "more");
 menubaritem.click();
 assert_false(menulist.matches(':popover-open'),
    'The menulist should not open because the menuitem command is invalid');
}, "Menuitem with invalid command/commandfor cannot invoke menulist popover.");

test(() => {
  assert_equals(checkableMenuitem.commandForElement,null,
   "To start, checkable item shouldn't be an invoker")
  opencheckable.click(); // Open the menu
  checkableMenuitem.click();
  assert_true(checkableMenuitem.checked,
      "checkable menu item becomes checked");
  assert_false(menulist.matches(":popover-open"),
      "not an invoker yet");

  checkableMenuitem.click();

  assert_false(checkableMenuitem.checked,
      "checkable menu item is no longer checked");
  assert_false(menulist.matches(":popover-open"),
      "menulist no longer matches :popover-open");

   // Being a sub-menu invoker makes the item non-checkable.
  checkableMenuitem.command = "toggle-menu";
  checkableMenuitem.commandForElement = menulist;
  checkableMenuitem.click();
  assert_false(checkableMenuitem.checked,
      "checkable menu item that invokes a menu is not checkable");
  assert_true(menulist.matches(":popover-open"), "menulist is open");
  checkableMenuitem.click();
  assert_false(checkableMenuitem.checked, "still not checked");
  assert_false(menulist.matches(":popover-open"), "menulist closes");
}, "menuitems that invoke menulists cannot be checkable");
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
        "byte_end": 251,
        "byte_start": 242,
        "col": 1,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menubar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 251,
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
        "byte_end": 281,
        "byte_start": 254,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 281,
        "byte_start": 254,
        "col": 3,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 318,
        "byte_start": 308,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 318,
        "byte_start": 308,
        "col": 3,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 413,
        "byte_start": 341,
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
        "byte_end": 413,
        "byte_start": 341,
        "col": 3,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 476,
        "byte_start": 456,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 476,
        "byte_start": 456,
        "col": 1,
        "line": 13
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 515,
        "byte_start": 478,
        "col": 2,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 515,
        "byte_start": 478,
        "col": 2,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 547,
        "byte_start": 537,
        "col": 2,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 547,
        "byte_start": 537,
        "col": 2,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 606,
        "byte_start": 581,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 606,
        "byte_start": 581,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 668,
        "byte_start": 634,
        "col": 5,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 668,
        "byte_start": 634,
        "col": 5,
        "line": 20
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
  "source_name": "html/semantics/menu/tentative/menubar-invoke-menulist.html"
}
```
