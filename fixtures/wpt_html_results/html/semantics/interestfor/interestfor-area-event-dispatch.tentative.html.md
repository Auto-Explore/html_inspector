# html/semantics/interestfor/interestfor-area-event-dispatch.tentative.html

Counts:
- errors: 2
- warnings: 5
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/interestfor/interestfor-area-event-dispatch.tentative.html",
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
<link rel=author href="mailto:masonf@chromium.org">
<link rel="help" href="https://open-ui.org/components/interest-invokers.explainer">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="resources/invoker-utils.js"></script>

<div id="interestee"></div>
<map id="map1"><area shape="default" id="nohref" interestfor="interestee"></map>
<img id=nohref_img src="/images/blue.png" usemap="#map1">
<map id="map2"><area href="/" shape="default" id="interestarea" interestfor="interestee"></map>
<img id=interestarea_img src="/images/blue.png" usemap="#map2">
<button id="otherbutton">Other Button</button>
<style>
  [interestfor] {
    interest-delay: 0s;
  }
</style>

<script>
  promise_test(async function (t) {
    t.add_cleanup(async () => {
      await hoverOver(otherbutton);
    });
    let gotEvent = false;
    interestee.addEventListener("interest", () => (gotEvent = true), { once: true });
    await hoverOver(nohref_img);
    assert_false(gotEvent, "InterestEvent should not get fired");
    nohref.setAttribute('href','foo');
    await hoverOver(nohref_img);
    assert_false(gotEvent, "adding href while the element is already hovered should not fire interest");
    await hoverOver(otherbutton);
    await hoverOver(nohref_img);
    assert_true(gotEvent, "interest should now be fired");
  }, "InterestEvent is not dispatched unless the area has an href");

  promise_test(async function (t) {
    t.add_cleanup(async () => {
      await hoverOver(otherbutton);
    });
    let event = null;
    interestee.addEventListener("interest", (e) => (event = e), { once: true });
    await hoverOver(interestarea_img);
    assert_true(!!event, "InterestEvent is fired");
    assert_true(event instanceof InterestEvent, "event is InterestEvent");
    assert_equals(event.type, "interest", "type");
    assert_equals(event.bubbles, false, "bubbles");
    assert_equals(event.composed, true, "composed");
    assert_equals(event.isTrusted, true, "isTrusted");
    assert_equals(event.target, interestee, "target");
    assert_equals(event.source, interestarea, "source");
  }, "InterestEvent dispatches on area hover");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 814,
        "byte_start": 757,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 974,
        "byte_start": 911,
        "col": 1,
        "line": 18
      }
    },
    {
      "category": "Html",
      "code": "html.style.not_allowed_here",
      "message": "Element “style” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 1029,
        "byte_start": 1022,
        "col": 1,
        "line": 20
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “map1”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 814,
        "byte_start": 757,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.img.usemap.missing_map_name",
      "message": "The hash-name reference in attribute “usemap” referred to “map2”, but there is no “map” element with a “name” attribute with that value.",
      "severity": "Error",
      "span": {
        "byte_end": 974,
        "byte_start": 911,
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
  "source_name": "html/semantics/interestfor/interestfor-area-event-dispatch.tentative.html"
}
```
