# html/semantics/the-button-element/command-and-commandfor/on-details-behavior.tentative.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-details-behavior.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<meta name="author" title="Luke Warlow" href="mailto:luke@warlow.dev" />
<meta name="timeout" content="long" />
<link rel="help" href="https://open-ui.org/components/invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<details id="invokee">Details Contents</details>
<button id="invokerbutton" commandfor="invokee" command="open"></button>

<script>
  function resetState() {
    invokerbutton.removeAttribute("command");
    invokee.removeAttribute("open");
  }

  // Open actions
  [
    "toggle",
    "open",
    /* test case sensitivity */
    "tOgGlE",
    "oPeN",
  ].forEach((command) => {
    promise_test(
      async function (t) {
        t.add_cleanup(resetState);
        invokerbutton.command = command;
        assert_false(invokee.matches("[open]"), "invokee does not match [open]");
        await clickOn(invokerbutton);
        assert_true(invokee.matches("[open]"), "invokee matches [open]");
      },
      `invoking (as ${command}) closed details opens`,
    );

    promise_test(
      async function (t) {
        t.add_cleanup(resetState);
        invokerbutton.command = command;
        assert_false(invokee.matches("[open]"), "invokee does not match [open]");
        invokee.addEventListener("command", (e) => e.preventDefault(), {
          once: true,
        });
        await clickOn(invokerbutton);
        t.add_cleanup(() => invokee.removeAttribute("open"));
        assert_false(invokee.matches("[open]"), "invokee still does not match [open]");
      },
      `invoking (as ${command}) closed details with preventDefault does not open`,
    );
  });

  // Close actions
  [
    "toggle",
    "close",
    /* test case sensitivity */
    "tOgGlE",
    "cLoSe",
  ].forEach((command) => {
    promise_test(
      async function (t) {
        t.add_cleanup(resetState);
        invokerbutton.command = command;
        invokee.setAttribute("open", "");
        assert_true(invokee.matches("[open]"));
        await clickOn(invokerbutton);
        assert_false(invokee.matches("[open]"));
      },
      `invoking (as ${command}) open details closes`,
    );

    promise_test(
      async function (t) {
        t.add_cleanup(resetState);
        invokerbutton.command = command;
        invokee.setAttribute("open", "");
        invokerbutton.setAttribute("command", "toggle");
        invokee.addEventListener("command", (e) => e.preventDefault(), {
          once: true,
        });
        assert_true(invokee.matches("[open]"));
        await clickOn(invokerbutton);
        assert_true(invokee.matches("[open]"));
      },
      `invoking (as ${command}) open details with prevent default closes`,
    );
  });

  // toggle specific

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokerbutton.command = "toggle";
    invokee.addEventListener(
      "command",
      (e) => {
        invokee.setAttribute("open", "");
      },
      {
        once: true,
      },
    );
    assert_false(invokee.matches("[open]"));
    await clickOn(invokerbutton);
    assert_false(invokee.matches("[open]"));
  }, "invoking (as toggle) closed details where event listener opens leads to a closed details");

  // open specific

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokerbutton.command = "open";
    invokee.setAttribute("open", "");
    assert_true(invokee.matches("[open]"));
    await clickOn(invokerbutton);
    assert_true(invokee.matches("[open]"));
  }, "invoking open details with open command is noop");

  // close

  promise_test(async function (t) {
    t.add_cleanup(resetState);
    invokerbutton.command = "close";
    assert_false(invokee.matches("[open]"));
    await clickOn(invokerbutton);
    assert_false(invokee.matches("[open]"));
  }, "invoking closed details with close command is noop");
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
        "byte_end": 113,
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
        "byte_end": 113,
        "byte_start": 41,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 599,
        "byte_start": 589,
        "col": 39,
        "line": 13
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-details-behavior.tentative.html"
}
```
