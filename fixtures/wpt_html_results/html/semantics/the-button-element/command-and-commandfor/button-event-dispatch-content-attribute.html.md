# html/semantics/the-button-element/command-and-commandfor/button-event-dispatch-content-attribute.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/the-button-element/command-and-commandfor/button-event-dispatch-content-attribute.html",
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

<button
  id="invokerbutton"
  commandfor="invokee"
  command="--custom-command"
></button>
<input
  type="number"
  id="invokee"
  oncommand="this.value = Number(this.value) + 1"
  value="0"
/>
<div popover id="popover" oncommand="return false"></div>

<script>
  test(function (t) {
    assert_true(
      typeof invokee.oncommand == "function",
      "invokee has an oncommand function",
    );
    assert_equals(invokee.valueAsNumber, 0, "input is equal to 0");
    invokerbutton.click();
    assert_equals(invokee.valueAsNumber, 1, "input is equal to 1");
  }, "oncommand content attribute works");

  test(function (t) {
    t.add_cleanup(() => {
      invokerbutton.command = "--custom-command";
    });
    invokerbutton.command = "show-popover";
    assert_true(
      typeof popover.oncommand == "function",
      "invokee has an oncommand function",
    );
    assert_false(popover.matches(":popover-open"), "popover is not open");
    invokerbutton.click();
    assert_false(popover.matches(":popover-open"), "popover is still not open");
  }, "oncommand content with a value of false prevents default");
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
  "source_name": "html/semantics/the-button-element/command-and-commandfor/button-event-dispatch-content-attribute.html"
}
```
