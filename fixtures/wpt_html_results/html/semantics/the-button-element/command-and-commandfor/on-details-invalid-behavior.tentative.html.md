# html/semantics/the-button-element/command-and-commandfor/on-details-invalid-behavior.tentative.html

Counts:
- errors: 0
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/on-details-invalid-behavior.tentative.html",
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

  // invalid actions on details
  [
    "",
    "foo-bar",
    "show-popover",
    "show-modal",
    "show-picker",
    "hide-popover",
    "hide",
    "toggle-open",
  ].forEach((command) => {
    promise_test(async function (t) {
      t.add_cleanup(resetState);
      invokerbutton.command = command;
      assert_false(invokee.matches("[open]"));
      await clickOn(invokerbutton);
      assert_false(invokee.matches("[open]"));
    }, `invoking (as ${command}) on details does nothing`);

    promise_test(async function (t) {
      t.add_cleanup(resetState);
      invokerbutton.command = command;
      invokee.setAttribute("open", "");
      assert_true(invokee.matches("[open]"));
      await clickOn(invokerbutton);
      assert_true(invokee.matches("[open]"));
    }, `invoking (as ${command}) on open details does nothing`);
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
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 606,
        "byte_start": 596,
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/on-details-invalid-behavior.tentative.html"
}
```
