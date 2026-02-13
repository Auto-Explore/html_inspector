# html/semantics/the-button-element/command-and-commandfor/on-dialog-invalid-behavior.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-dialog-invalid-behavior.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Keith Cirkel" href="mailto:wpt@keithcirkel.co.uk" />
<meta name="timeout" content="long">
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/invoker-utils.js"></script>

<dialog id="invokee">
  <button id="containedinvoker" commandfor="invokee" command="close"></button>
</dialog>
<button id="invokerbutton" commandfor="invokee" command="showmodal"></button>

<script>
  function resetState() {
    invokee.close();
    try { invokee.hidePopover(); } catch {}
    invokee.removeAttribute("popover");
    invokerbutton.removeAttribute("command");
    containedinvoker.removeAttribute("command");
  }

  // invalid
  [
    "",
    "foo",
    "foo-bar",
    "auto",
    "showmodal",
    "show-picker",
  ].forEach((action) => {
    test(function (t) {
      t.add_cleanup(resetState);
      let command_event_fired = false;
      invokee.addEventListener("command", () => (command_event_fired = true), { signal: t.get_signal() });
      invokerbutton.setAttribute("command", action);
      assert_false(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      invokerbutton.click();
      assert_false(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      assert_false(command_event_fired, "command event fired");
    }, `invoking (as ${action}) on dialog does nothing`);

    test(function (t) {
      t.add_cleanup(resetState);
      let command_event_fired = false;
      invokee.addEventListener("command", () => (command_event_fired = true), { signal: t.get_signal() });
      containedinvoker.setAttribute("command", action);
      invokee.show();
      assert_true(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      containedinvoker.click();
      assert_true(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      assert_false(command_event_fired, "command event fired");
    }, `invoking (as ${action}) on open dialog does nothing`);

    test(function (t) {
      t.add_cleanup(resetState);
      let command_event_fired = false;
      invokee.addEventListener("command", () => (command_event_fired = true), { signal: t.get_signal() });
      containedinvoker.setAttribute("command", action);
      invokee.showModal();
      assert_true(invokee.open, "invokee.open");
      assert_true(invokee.matches(":modal"), "invokee :modal");
      containedinvoker.click();
      assert_true(invokee.open, "invokee.open");
      assert_true(invokee.matches(":modal"), "invokee :modal");
      assert_false(command_event_fired, "command event fired");
    }, `invoking (as ${action}) on open modal dialog does nothing`);

    test(function (t) {
      t.add_cleanup(resetState);
      containedinvoker.setAttribute("command", action);
      invokee.showModal();
      assert_true(invokee.open, "invokee.open");
      assert_true(invokee.matches(":modal"), "invokee :modal");
      invokee.addEventListener(
        "command",
        (e) => {
          containedinvoker.setAttribute("command", "");
        },
        { once: true },
      );
      containedinvoker.click();
      assert_true(invokee.open, "invokee.open");
      assert_true(invokee.matches(":modal"), "invokee :modal");
    }, `invoking (as ${action}) on open modal while changing the attributer does nothing`);
  });

  // popover commands on dialog
  [
    "show-popover",
    "hide-popover",
    "toggle-popover",
  ].forEach((action) => {
    test(function (t) {
      t.add_cleanup(resetState);
      let command_event_fired = false;
      invokee.addEventListener("command", () => (command_event_fired = true), { signal: t.get_signal() });
      invokerbutton.setAttribute("command", action);
      assert_false(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      invokerbutton.click();
      assert_false(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      assert_true(command_event_fired, "command event fired");
    }, `invoking (as ${action}) on dialog fires event`);

    test(function (t) {
      t.add_cleanup(resetState);
      let command_event_fired = false;
      invokee.addEventListener("command", () => (command_event_fired = true), { signal: t.get_signal() });
      containedinvoker.setAttribute("command", action);
      invokee.show();
      assert_true(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      containedinvoker.click();
      assert_true(invokee.open, "invokee.open");
      assert_false(invokee.matches(":modal"), "invokee :modal");
      assert_true(command_event_fired, "command event fired");
    }, `invoking (as ${action}) on open dialog fires event`);

    test(function (t) {
      t.add_cleanup(resetState);
      let command_event_fired = false;
      invokee.addEventListener("command", () => (command_event_fired = true), { signal: t.get_signal() });
      containedinvoker.setAttribute("command", action);
      invokee.showModal();
      assert_true(invokee.open, "invokee.open");
      assert_true(invokee.matches(":modal"), "invokee :modal");
      containedinvoker.click();
      assert_true(invokee.open, "invokee.open");
      assert_true(invokee.matches(":modal"), "invokee :modal");
      assert_true(command_event_fired, "command event fired");
    }, `invoking (as ${action}) on open modal dialog fires event`);

    test(function (t) {
      t.add_cleanup(resetState);
      containedinvoker.setAttribute("command", action);
      invokee.showModal();
      assert_true(invokee.open, "invokee.open");
      assert_true(invokee.matches(":modal"), "invokee :modal");
      invokee.addEventListener(
        "command",
        (e) => {
          containedinvoker.setAttribute("command", "");
        },
        { once: true },
      );
      containedinvoker.click();
      assert_true(invokee.open, "invokee.open");
      assert_true(invokee.matches(":modal"), "invokee :modal");
    }, `invoking (as ${action}) on open modal while changing the attributer does nothing`);
  });

  // Open Popovers using Dialog actions
  ["show-modal", "close"].forEach((action) => {
    ["manual", "auto"].forEach((popoverState) => {
      test(
        function (t) {
          t.add_cleanup(resetState);
          invokee.setAttribute("popover", popoverState);
          invokee.showPopover();
          containedinvoker.setAttribute("command", action);
          assert_true(
            invokee.matches(":popover-open"),
            "invokee :popover-open",
          );
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
          invokee.addEventListener("command", (e) => e.preventDefault(), {
            once: true,
          });
          containedinvoker.click();
          assert_true(
            invokee.matches(":popover-open"),
            "invokee :popover-open",
          );
          assert_false(invokee.open, "invokee.open");
          assert_false(invokee.matches(":modal"), "invokee :modal");
        },
        `invoking (as ${
          action || "explicit empty"
        }) dialog as open popover=${popoverState} is noop`,
      );
    });
  });
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-dialog-invalid-behavior.html"
}
```
