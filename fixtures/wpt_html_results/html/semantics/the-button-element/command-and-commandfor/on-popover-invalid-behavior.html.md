# html/semantics/the-button-element/command-and-commandfor/on-popover-invalid-behavior.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-popover-invalid-behavior.html",
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
  <button id="containedinvoker" commandfor="invokee"></button>
</div>
<button id="invokerbutton" commandfor="invokee"></button>

<script>
  function resetState() {
    invokerbutton.removeAttribute("command");
    containedinvoker.removeAttribute("command");
    try {
      invokee.hidePopover();
    } catch {}
    invokee.setAttribute("popover", "");
  }

  // invalid actions on show-popover
  [null, "", "foo-bar", "showpopover", "show-modal", "show-picker", "open", "close"].forEach((command) => {
    test(function (t) {
      t.add_cleanup(resetState);
      let command_event_fired = false;
      invokee.addEventListener("command", () => (command_event_fired = true), { signal: t.get_signal() });
      invokerbutton.command = command;
      assert_false(invokee.matches(":popover-open"));
      invokerbutton.click();
      assert_false(invokee.matches(":popover-open"));
      assert_false(command_event_fired, "command event fired");
    }, `invoking (as ${command}) on popover does nothing`);

    test(function (t) {
      t.add_cleanup(resetState);
      let command_event_fired = false;
      invokee.addEventListener("command", () => (command_event_fired = true), { signal: t.get_signal() });
      invokerbutton.command = command;
      invokee.showPopover();
      assert_true(invokee.matches(":popover-open"));
      invokerbutton.click();
      assert_true(invokee.matches(":popover-open"));
      assert_false(command_event_fired, "command event fired");
    }, `invoking (as ${command}) on open popover does nothing`);
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-popover-invalid-behavior.html"
}
```
