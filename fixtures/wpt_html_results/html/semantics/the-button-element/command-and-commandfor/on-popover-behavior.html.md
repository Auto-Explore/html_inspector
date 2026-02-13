# html/semantics/the-button-element/command-and-commandfor/on-popover-behavior.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-popover-behavior.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<meta name="timeout" content="long" />
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/invoker-utils.js"></script>

<div id="invokee" popover>
  <button id="containedinvoker" commandfor="invokee" command="hide-popover"></button>
</div>
<button id="invokerbutton" commandfor="invokee" command="toggle-popover"></button>

<style>
#invokee {
  margin: 0;
  position-area: block-end span-all;
}
</style>

<script>
  function resetState() {
    invokerbutton.setAttribute("commandfor", "invokee");
    invokerbutton.setAttribute("command", "toggle-popover");
    containedinvoker.setAttribute("commandfor", "invokee");
    containedinvoker.setAttribute("command", "hide-popover");
    try {
      invokee.hidePopover();
    } catch {}
    invokee.setAttribute("popover", "");
  }

  promise_test(async function (t) {
    assert_false(invokee.matches(":popover-open"));
    invokee.addEventListener("command", (e) => { invokerbutton.setAttribute('command', 'hide-popover'); }, {
      once: true,
    });
    invokerbutton.click();
    t.add_cleanup(resetState);
    assert_true(invokee.matches(":popover-open"));
  }, "changing command attribute inside invokeevent doesn't impact the invocation");

  // Open actions
  [
    "toggle-popover",
    "show-popover",
    /* test case sensitivity */
    "tOgGlE-pOpOvEr",
    "sHoW-pOpOvEr",
  ].forEach((command) => {
    test(
      function (t) {
        t.add_cleanup(resetState);
        invokerbutton.command = command;
        assert_false(invokee.matches(":popover-open"));
        invokerbutton.click();
        assert_true(invokee.matches(":popover-open"));
      },
      `invoking (as ${command}) closed popover opens`,
    );

    test(
      function (t) {
        t.add_cleanup(resetState);
        invokerbutton.command = command;
        assert_false(invokee.matches(":popover-open"));
        invokee.addEventListener("command", (e) => e.preventDefault(), {
          once: true,
        });
        invokerbutton.click();
        assert_false(invokee.matches(":popover-open"));
      },
      `invoking (as ${command}) closed popover with preventDefault does not open`,
    );
  });

  // Close actions
  [
    "toggle-popover",
    "hide-popover",
    /* test case sensitivity */
    "tOgGlE-pOpOvEr",
    "hIdE-pOpOvEr",
  ].forEach((command) => {
    test(
      function (t) {
        t.add_cleanup(resetState);
        invokerbutton.command = command;
        invokee.showPopover();
        assert_true(invokee.matches(":popover-open"));
        invokerbutton.click();
        assert_false(invokee.matches(":popover-open"));
      },
      `invoking (as ${command}) open popover closes`,
    );

    test(
      function (t) {
        t.add_cleanup(resetState);
        invokerbutton.command = command;
        invokee.showPopover();
        assert_true(invokee.matches(":popover-open"));
        invokee.addEventListener("command", (e) => e.preventDefault(), {
          once: true,
        });
        invokerbutton.click();
        assert_true(invokee.matches(":popover-open"));
      },
      `invoking (as ${command}) open popover with preventDefault does not close`,
    );

    test(
      function (t) {
        t.add_cleanup(resetState);
        containedinvoker.command = command;
        invokee.showPopover();
        assert_true(invokee.matches(":popover-open"));
        containedinvoker.click();
        assert_false(invokee.matches(":popover-open"));
      },
      `invoking (as ${command}) from within open popover closes`,
    );

    test(
      function (t) {
        t.add_cleanup(resetState);
        containedinvoker.command = command;
        invokee.showPopover();
        invokee.addEventListener("command", (e) => e.preventDefault(), {
          once: true,
        });
        assert_true(invokee.matches(":popover-open"));
        containedinvoker.click();
        assert_true(invokee.matches(":popover-open"));
      },
      `invoking (as ${command}) from within open popover with preventDefault does not close`,
    );
  });

  // show-popover specific
  test(function (t) {
    t.add_cleanup(resetState);
    invokerbutton.setAttribute("command", "show-popover");
    invokee.showPopover();
    assert_true(invokee.matches(":popover-open"));
    invokerbutton.click();
    assert_true(invokee.matches(":popover-open"));
  }, "invoking (as show-popover) open popover is noop");

  // hide-popover specific
  test(function (t) {
    t.add_cleanup(resetState);
    invokerbutton.setAttribute("command", "hide-popover");
    assert_false(invokee.matches(":popover-open"));
    invokerbutton.click();
    assert_false(invokee.matches(":popover-open"));
  }, "invoking (as hide-popover) closed popover is noop");


  test(function (t) {
    t.add_cleanup(resetState);
    invokerbutton.setAttribute("command", "show-popover");
    invokerbutton.click();
    assert_true(invokee.matches(":popover-open"),
      ":popover-open should match after calling invokerbutton.click()");
    const rect = invokee.getBoundingClientRect();
    assert_not_equals(rect.y, 0,
      "popover should not be at the top of the window because it is anchor positioned.");
  }, "invoking (as show-popover) should create an implicit anchor reference for anchor positioning.");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.attr.href.not_allowed",
      "message": "Attribute “href” not allowed on element “meta” at this point.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.meta.missing_content",
      "message": "Element “meta” is missing one or more of the following attributes: “content”, “property”.",
      "severity": "Warning",
      "span": {
        "byte_end": 120,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 607,
        "byte_start": 600,
        "col": 1,
        "line": 15
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-popover-behavior.html"
}
```
