# html/semantics/interestfor/interestfor-on-popover-behavior.tentative.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-on-popover-behavior.tentative.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8" />
<link rel="author" title="Keith Cirkel" href="mailto:keithamus@github.com" >
<link rel="author" title="Luke Warlow" href="mailto:lwarlow@igalia.com" >
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer/" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<div id="interestee" popover>
  Popover Content
</div>
<button id="interestbutton" interestfor="interestee">Interest Invoker</button>
<button id="otherbutton">Other button</button>
<style>
  [interestfor] {
    interest-delay: 0s;
  }
</style>

<script>
  async function reset() {
      await hoverOver(otherbutton);
      interestee.hidePopover();
  }
  promise_test(async function (t) {
    t.add_cleanup(reset);
    assert_false(interestee.matches(":popover-open"));
    await hoverOver(interestbutton);
    assert_true(interestee.matches(":popover-open"));
  }, "hover interest invoking closed popover opens");

  promise_test(async function (t) {
    t.add_cleanup(reset);
    assert_false(interestee.matches(":popover-open"));
    interestee.addEventListener("interest", (e) => e.preventDefault(), {
      once: true,
    });
    await hoverOver(interestbutton);
    assert_false(interestee.matches(":popover-open"));
  }, "interest invoking closed popover with preventDefault does not open");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 787,
        "byte_start": 780,
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
  "source_name": "html/semantics/interestfor/interestfor-on-popover-behavior.tentative.html"
}
```
