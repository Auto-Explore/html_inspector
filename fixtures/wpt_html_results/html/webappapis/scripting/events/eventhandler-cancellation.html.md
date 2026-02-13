# html/webappapis/scripting/events/eventhandler-cancellation.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/eventhandler-cancellation.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title></title>
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>
<!-- A window to work with that won't trigger the harness exception detection
     when we fire "error" events at it -->
<iframe style="display: none"></iframe>
<script>
  test(function() {
    var blob = new Blob([""]);
    // Most targets disabled for now until
    // https://github.com/whatwg/html/issues/2296 is sorted out.
    var targets = [ frames[0] /*, document, document.documentElement,
                    new Worker(URL.createObjectURL(blob) */ ];
    // Event constructors also mostly disabled until
    // https://github.com/whatwg/html/issues/2296 is sorted out.
    var eventCtors = [ /* Event, */ ErrorEvent /*, MouseEvent */ ];
    var values = [true, false, "", "abc", {}, 0, 1, -1, null, undefined,
                  2.5, NaN, Infinity, Symbol.toStringTag ];
    // Event types also mostly disabled pending
    // https://github.com/whatwg/html/issues/2296
    var eventTypes = [ "error"/*, "click", "load"*/ ];

    // Variables that keep track of which subtest we're running.
    var curTarget;
    var curValue;
    var curCtor;
    var curType;

    function defaultPreventedTester(event) {
      var expectedValue;
      if (curTarget === frames[0] &&
          curCtor === ErrorEvent &&
          curValue === true &&
          curType == "error") {
        expectedValue = true;
      } else {
        // This will need adjusting once we allow more targets and event
        // constructors above!
        expectedValue = false;
      }
      var valueRepr;
      if (typeof curValue == "string") {
        valueRepr = '"' + curValue + '"';
      } else {
        valueRepr = String(curValue);
      }
      test(function() {
        assert_equals(event.defaultPrevented, expectedValue);
      }, "Returning " + valueRepr +
         " from " + String(curTarget) + "'s on" + curType +
         " event handler while " + curCtor.name +
         " is firing should" +
         (expectedValue ? "" : " not") +
         " cancel the event");
    }

    for (curCtor of eventCtors) {
      for (curTarget of targets) {
        for (curType of eventTypes) {
          for (curValue of values) {
            // We have to make sure that defaultPreventedTester is added after
            // our event handler.
            curTarget["on" + curType] = function() { return curValue; }
            curTarget.addEventListener(curType, defaultPreventedTester);
            var e = new curCtor(curType, { cancelable: true });
            curTarget.dispatchEvent(e);
            curTarget["on" + curType] = null;
            curTarget.removeEventListener(curType, defaultPreventedTester);
          }
        }
      }
    }
  }, "event handler cancellation behavior");
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.title.empty",
      "message": "Element “title” must not be empty.",
      "severity": "Warning",
      "span": {
        "byte_end": 44,
        "byte_start": 37,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/scripting/events/eventhandler-cancellation.html"
}
```
