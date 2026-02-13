# html/editing/the-hidden-attribute/beforematch-attribute-removal-001.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/the-hidden-attribute/beforematch-attribute-removal-001.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<link rel="author" title="Tim Nguyen" href="https://github.com/nt1m">
<link rel="help" href="https://html.spec.whatwg.org/#ancestor-revealing-algorithm">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<details id="a3">
    <div id="a2" hidden="until-found">
        <details id="a1" hidden="until-found">
            <div id="a1child">Hidden</div>
        </details>
    </div>
</details>

<script>
function test_state({ a1open, a1hidden, a2hidden, a3open }) {
    assert_equals(a1.open, a1open, `a1 should ${a1open ? "" : "not "}be open`);
    assert_equals(a1.hidden, a1hidden ? "until-found" : false, `a1 should ${a1hidden ? "" : "not "}be hidden`);
    assert_equals(a2.hidden, a2hidden ? "until-found" : false, `a2 should ${a2hidden ? "" : "not "}be hidden`);
    assert_equals(a3.open, a3open, `a3 should ${a3open ? "" : "not "}be open`);
}
t = async_test("hidden=until-found and details revealing algorithm should abort if attribute states are mutated on beforematch events.");
test_state({
    a1open: false,
    a1hidden: true,
    a2hidden: true,
    a3open: false
});
a1.addEventListener("beforematch", t.step_func(() => {
    test_state({
        a1open: true, // We find the <details> element before finding hidden=until-found as a consequence of tree-traversal order.
        a1hidden: true, // hidden=until-found removal happens after beforematch event.
        a2hidden: true,
        a3open: false
    });
    a2.addEventListener("beforematch", t.step_func((e) => {
        assert_equals(e.target, a1, "a1 beforematch event bubbles up");
        // No change in state, since it's part of the same event dispatch as above.
        test_state({
            a1open: true,
            a1hidden: true,
            a2hidden: true,
            a3open: false
        });
        a1.hidden = true;
        a2.addEventListener("beforematch", t.unreached_func("Algorithm should have aborted due to hidden attribute no longer being in the until-found state."));
        a3.addEventListener("toggle", t.unreached_func("Algorithm should have aborted due to hidden attribute no longer being in the until-found state."));
        t.done();
    }), { once: true });
}), { once: true });

location.hash = "#a1child";
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 442,
        "byte_start": 432,
        "col": 9,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.details.missing_summary",
      "message": "Element “details” is missing a required instance of child element “summary”.",
      "severity": "Warning",
      "span": {
        "byte_end": 464,
        "byte_start": 454,
        "col": 1,
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
  "source_name": "html/editing/the-hidden-attribute/beforematch-attribute-removal-001.html"
}
```
