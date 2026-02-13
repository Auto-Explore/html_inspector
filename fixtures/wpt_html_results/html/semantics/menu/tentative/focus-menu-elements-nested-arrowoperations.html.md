# html/semantics/menu/tentative/focus-menu-elements-nested-arrowoperations.html

Counts:
- errors: 0
- warnings: 34
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/focus-menu-elements-nested-arrowoperations.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta name="timeout" content="long">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<link rel=author href=mailto:dizhangg@chromium.org>
<link rel=help href=https://open-ui.org/components/menu.explainer>

<style>
/* TODO: Remove CSS rule when it is defined in UA style sheet. */
menulist {
  position-area: bottom span-right;
  position: absolute;
}

.nested {
  position-area: right span-bottom;
}
</style>

<div id="container">
 <menubar>
   <menuitem id="a" command="toggle-menu" commandfor="menulist1">Commands A</menuitem>
   <menuitem id="b" command="toggle-menu" commandfor="menulist2">Commands B</menuitem>
   <menuitem id="c">Commands C</menuitem>
 </menubar>
</div>

<menulist id="menulist1">
 <menuitem id="d">Command D</menuitem>
</menulist>

<menulist id="menulist2">
 <menuitem id="e" command="show-menu" commandfor="menulist4">Command E</menuitem>
 <menuitem id="f" command="hide-menu" commandfor="menulist4">Command F</menuitem>
 <menuitem id="g" command="toggle-menu" commandfor="menulist3">Commands G</menuitem>
</menulist>

<menulist id="menulist3" class="nested">
 <menuitem id="h">Command H</menuitem>
 <menuitem id="i">Command I</menuitem>
</menulist>

<menulist id="menulist4" class="nested">
 <menuitem id="j">Command J</menuitem>
 <menuitem id="k">Command K</menuitem>
</menulist>

<script>

const ArrowLeft = '\uE012';
const ArrowUp = '\uE013';
const ArrowRight = '\uE014';
const ArrowDown = '\uE015';

async function navigateMenus() {
  await test_driver.click(a);
  assert_equals(document.activeElement, a);
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement, d);
  await test_driver.send_keys(document.activeElement, ArrowRight);
  assert_equals(document.activeElement, b);
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement, e);
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement, f);
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement, g);
  await test_driver.send_keys(document.activeElement, ArrowRight);
  assert_equals(document.activeElement, h);
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement, i);
  await test_driver.send_keys(document.activeElement, ArrowRight);
  assert_equals(document.activeElement, c);
  await test_driver.send_keys(document.activeElement, ArrowLeft);
  assert_equals(document.activeElement, b);
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement, e);
  await test_driver.send_keys(document.activeElement, ArrowRight);
  assert_equals(document.activeElement, c, "show-menu does not create special submenu relationship");
  await test_driver.send_keys(document.activeElement, ArrowLeft);
  assert_equals(document.activeElement, b);
  await test_driver.send_keys(document.activeElement, ArrowDown);
  await test_driver.send_keys(document.activeElement, ArrowDown);
  assert_equals(document.activeElement, f);
  await test_driver.send_keys(document.activeElement, ArrowRight);
  assert_equals(document.activeElement, c, "hide-menu does not create special submenu relationship");
}

promise_test(async (t) => {
  await navigateMenus();
}, 'Should use arrow keys to move between menuitems in menulist invoked from menubar.');

promise_test(async (t) => {
  container.setAttribute('popover','auto');
  container.showPopover();
  await navigateMenus();
  container.removeAttribute('popover');
}, 'Should use arrow keys to move between menuitems in menulist invoked from menubar inside a popover without closing the popover.');

promise_test(async (t) => {
  document.querySelectorAll("menulist").forEach((menulist) => {
   menulist.addEventListener('beforetoggle', () => menulist.hidePopover());
  });
  a.focus();
  await navigateMenus();
}, 'Should use arrow keys to move between menuitems: Hide popover on beforetoggle should still work.');

</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menubar” not allowed as child of “div” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 619,
        "byte_start": 610,
        "col": 2,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menubar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 619,
        "byte_start": 610,
        "col": 2,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 685,
        "byte_start": 623,
        "col": 4,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 685,
        "byte_start": 623,
        "col": 4,
        "line": 24
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 772,
        "byte_start": 710,
        "col": 4,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 772,
        "byte_start": 710,
        "col": 4,
        "line": 25
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 814,
        "byte_start": 797,
        "col": 4,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 814,
        "byte_start": 797,
        "col": 4,
        "line": 26
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 881,
        "byte_start": 856,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 881,
        "byte_start": 856,
        "col": 1,
        "line": 30
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 900,
        "byte_start": 883,
        "col": 2,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 900,
        "byte_start": 883,
        "col": 2,
        "line": 31
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 959,
        "byte_start": 934,
        "col": 1,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 959,
        "byte_start": 934,
        "col": 1,
        "line": 34
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1021,
        "byte_start": 961,
        "col": 2,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1021,
        "byte_start": 961,
        "col": 2,
        "line": 35
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1103,
        "byte_start": 1043,
        "col": 2,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1103,
        "byte_start": 1043,
        "col": 2,
        "line": 36
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1187,
        "byte_start": 1125,
        "col": 2,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1187,
        "byte_start": 1125,
        "col": 2,
        "line": 37
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1262,
        "byte_start": 1222,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1262,
        "byte_start": 1222,
        "col": 1,
        "line": 40
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1281,
        "byte_start": 1264,
        "col": 2,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1281,
        "byte_start": 1264,
        "col": 2,
        "line": 41
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1320,
        "byte_start": 1303,
        "col": 2,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1320,
        "byte_start": 1303,
        "col": 2,
        "line": 42
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1394,
        "byte_start": 1354,
        "col": 1,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1394,
        "byte_start": 1354,
        "col": 1,
        "line": 45
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1413,
        "byte_start": 1396,
        "col": 2,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1413,
        "byte_start": 1396,
        "col": 2,
        "line": 46
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 1452,
        "byte_start": 1435,
        "col": 2,
        "line": 47
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 1452,
        "byte_start": 1435,
        "col": 2,
        "line": 47
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
  "source_name": "html/semantics/menu/tentative/focus-menu-elements-nested-arrowoperations.html"
}
```
