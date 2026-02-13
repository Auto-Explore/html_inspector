# html/semantics/menu/tentative/menuitem-interest-invoker.tentative.html

Counts:
- errors: 0
- warnings: 7
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/menu/tentative/menuitem-interest-invoker.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<link rel=author href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="../../interestfor/resources/invoker-utils.js"></script>

<div id="interestee"></div>
<menulist id="menu">
  <menuitem id="interestmenuitem" interestfor="interestee">menuitem</menuitem>
</menulist>
<button id="otherbutton">Other Button</button>

<style>
  [interestfor] {
    interest-delay: 0s;
  }
</style>

<script>
  promise_test(async function (t) {
    const interestmenuitem = document.getElementById('interestmenuitem');
    const menu = document.getElementById('menu');
    menu.showPopover(); // So the menuitem is visible for hovering.
    t.add_cleanup(() => {
      menu.hidePopover();
    });

    let event = null;
    interestee.addEventListener("interest", (e) => (event = e));
    await hoverOver(interestmenuitem);
    assert_true(!!event, "InterestEvent is fired");
    assert_true(event instanceof InterestEvent, "event is InterestEvent");
    assert_equals(event.type, "interest", "type");
    assert_false(event.bubbles, "bubbles");
    assert_true(event.composed, "composed");
    assert_true(event.isTrusted, "isTrusted");
    assert_equals(event.target, interestee, "target");
    assert_equals(event.source, interestmenuitem, "source");
  }, "InterestEvent dispatches on menuitem hover");

  promise_test(async function (t) {
    const interestmenuitem = document.getElementById('interestmenuitem');
    const menu = document.getElementById('menu');
    menu.showPopover();

    t.add_cleanup(async () => {
      interestmenuitem.removeAttribute('disabled');
      await hoverOver(otherbutton);
      menu.hidePopover();
    });

    let called = false;
    const listener = () => { called = true; };
    interestee.addEventListener("interest", listener);
    t.add_cleanup(() => interestee.removeEventListener("interest", listener));


    interestmenuitem.setAttribute("disabled", "");
    await hoverOver(interestmenuitem);
    assert_false(called, "event was not called");
  }, "event does not dispatch if invoker menuitem is disabled");
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
        "byte_end": 563,
        "byte_start": 543,
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
        "byte_end": 563,
        "byte_start": 543,
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
        "byte_end": 623,
        "byte_start": 566,
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
        "byte_end": 623,
        "byte_start": 566,
        "col": 3,
        "line": 14
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 710,
        "byte_start": 703,
        "col": 1,
        "line": 18
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
  "source_name": "html/semantics/menu/tentative/menuitem-interest-invoker.tentative.html"
}
```
