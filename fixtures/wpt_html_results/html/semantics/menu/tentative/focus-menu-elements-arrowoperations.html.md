# html/semantics/menu/tentative/focus-menu-elements-arrowoperations.html

Counts:
- errors: 0
- warnings: 23
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/focus-menu-elements-arrowoperations.html",
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

<menubar>
 <menuitem id=btn commandfor="list" command="toggle-menu">Open</menuitem>
</menubar>

<menulist id="list">
 <menuitem id="A1">Command A1</menuitem>
 <menuitem id="A2">Command A2</menuitem>
 <menuitem id="A3">Command A3</menuitem>
</menulist>

<menubar id="bar">
  <menuitem id="B1">Command B1</menuitem>
  <menuitem id="B2">Command B2</menuitem>
  <menuitem id="B3">Command B3</menuitem>
</menubar>

<style>
  .nodelay menuitem {
    interest-delay: 0s;
  }
  .longdelay menuitem {
    interest-delay: 1000s;
  }
</style>

<script>

const Enter = '\uE007';
const ArrowLeft = '\uE012';
const ArrowUp = '\uE013';
const ArrowRight = '\uE014';
const ArrowDown = '\uE015';

[false, true].forEach(useInterest => {
  document.body.className = useInterest ? 'nodelay' : 'longdelay';
  const description = useInterest ? ' (no delays)' : ' (long delays)';
  promise_test(async (t) => {
    await test_driver.click(btn);
    assert_equals(document.activeElement, btn);
    await test_driver.send_keys(document.activeElement, Enter);
    assert_equals(document.activeElement, btn, 'btn invoked menulist, but focus is still on btn.');
    await test_driver.send_keys(document.activeElement, ArrowDown);
    assert_equals(document.activeElement, A1, 'arrow down moves focus into menulist.');
    await test_driver.send_keys(document.activeElement, ArrowDown);
    assert_equals(document.activeElement, A2, 'arrow down changes to next menuitem.');
    await test_driver.send_keys(document.activeElement, ArrowUp);
    assert_equals(document.activeElement, A1, 'arrow up changes to previous menuitem.');
    await test_driver.send_keys(document.activeElement, ArrowUp);
    assert_equals(document.activeElement, A3, 'arrow up key loops.');
    await test_driver.send_keys(document.activeElement, ArrowDown);
    assert_equals(document.activeElement, A1, 'arrow down key loops.');

    list.hidePopover();
    await test_driver.click(btn);
    await test_driver.send_keys(document.activeElement, ArrowDown);
    assert_equals(document.activeElement, A1, 'arrow down moves focus into menulist.');
    await test_driver.send_keys(document.activeElement, ArrowLeft);
    assert_equals(document.activeElement, btn, 'arrow left close the menulist and focus on invoker.');

    await test_driver.click(btn);
    await test_driver.send_keys(document.activeElement, ArrowDown);
    assert_equals(document.activeElement, A1, 'arrow down moves focus into menulist.');
    await test_driver.send_keys(document.activeElement, ArrowRight);
    assert_equals(document.activeElement, btn, 'arrow right close the menulist and focus on invoker.');
  }, `Should use arrow keys to move between menuitems in menulist${description}`);

  promise_test(async (t) => {
    list.style.display = 'block';
    await test_driver.click(btn);
    await test_driver.send_keys(document.activeElement, ArrowDown);
    assert_equals(document.activeElement, A1, 'arrow down moves focus into menulist.');
    await test_driver.send_keys(document.activeElement, ArrowDown);
    assert_equals(document.activeElement, A2, 'arrow down changes to next menuitem.');
    await test_driver.send_keys(document.activeElement, ArrowUp);
    assert_equals(document.activeElement, A1, 'arrow up changes to previous menuitem.');
    await test_driver.send_keys(document.activeElement, ArrowLeft);
    assert_equals(document.activeElement, btn, 'arrow left moves focus from menulist to invoker.');
    list.style.display = '';
  }, `Should use arrow keys to move between menuitems in menulist with display block${description}`);

  promise_test(async (t) => {
    await test_driver.click(B1);
    assert_equals(document.activeElement, B1);
    await test_driver.send_keys(document.activeElement, ArrowUp);
    assert_equals(document.activeElement, B1, 'arrow up does not change current focused element.');
    await test_driver.send_keys(document.activeElement, ArrowDown);
    assert_equals(document.activeElement, B1, 'arrow down does not change current focused element.');
    await test_driver.send_keys(document.activeElement, ArrowRight);
    assert_equals(document.activeElement, B2, 'arrow right changes to next menuitem.');
    await test_driver.send_keys(document.activeElement, ArrowLeft);
    assert_equals(document.activeElement, B1, 'arrow left changes to previous menuitem.');
    await test_driver.send_keys(document.activeElement, ArrowLeft);
    assert_equals(document.activeElement, B3, 'arrow left key loops.');
    await test_driver.send_keys(document.activeElement, ArrowRight);
    assert_equals(document.activeElement, B1, 'arrow right key loops.');
  }, `Should use arrow keys to move between menuitems in menubar${description}`);
});

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
        "byte_end": 393,
        "byte_start": 384,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menubar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 393,
        "byte_start": 384,
        "col": 1,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 452,
        "byte_start": 395,
        "col": 2,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 452,
        "byte_start": 395,
        "col": 2,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menulist” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 500,
        "byte_start": 480,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menulist” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 500,
        "byte_start": 480,
        "col": 1,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 520,
        "byte_start": 502,
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
        "byte_end": 520,
        "byte_start": 502,
        "col": 2,
        "line": 15
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 561,
        "byte_start": 543,
        "col": 2,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 561,
        "byte_start": 543,
        "col": 2,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 602,
        "byte_start": 584,
        "col": 2,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 602,
        "byte_start": 584,
        "col": 2,
        "line": 17
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.not_allowed",
      "message": "Element “menubar” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 655,
        "byte_start": 637,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menubar” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 655,
        "byte_start": 637,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 676,
        "byte_start": 658,
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
        "byte_end": 676,
        "byte_start": 658,
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
        "byte_end": 718,
        "byte_start": 700,
        "col": 3,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.element.menuitem.obsolete",
      "message": "The “menuitem” element is obsolete. Use script to handle “contextmenu” event instead.",
      "severity": "Warning",
      "span": {
        "byte_end": 718,
        "byte_start": 700,
        "col": 3,
        "line": 22
      }
    },
    {
      "category": "Html",
      "code": "html.unknown_element.completely_unknown",
      "message": "The “menuitem” element is a completely-unknown element that is not allowed anywhere in any HTML content.",
      "severity": "Warning",
      "span": {
        "byte_end": 760,
        "byte_start": 742,
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
        "byte_end": 760,
        "byte_start": 742,
        "col": 3,
        "line": 23
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 801,
        "byte_start": 794,
        "col": 1,
        "line": 26
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
  "source_name": "html/semantics/menu/tentative/focus-menu-elements-arrowoperations.html"
}
```
