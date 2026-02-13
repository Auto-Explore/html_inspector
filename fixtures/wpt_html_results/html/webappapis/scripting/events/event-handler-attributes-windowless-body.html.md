# html/webappapis/scripting/events/event-handler-attributes-windowless-body.html

Counts:
- errors: 7
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/event-handler-attributes-windowless-body.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title></title>
<body></body>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/WebIDLParser.js"></script>
<script src="resources/event-handler-body.js"></script>
<script>
setup({ explicit_done: true });
const elements = ['body', 'frameset'];
handlersListPromise.then(({ shadowedHandlers, notShadowedHandlers }) => {
  elements.forEach(function (elementName) {
    shadowedHandlers.forEach(function (eventName) {
      var handlerName = "on" + eventName;

      test(function() {
        var windowHandler = function () { return "Handler attached to the window"; };
        window[handlerName] = windowHandler;

        var d = (new DOMParser).parseFromString('', 'text/html');
        var b = d.createElement(elementName);

        assert_equals(b[handlerName], null);

        window[handlerName] = null;
      }, "Return null when getting the " + eventName + " event handler of a windowless " + elementName);

      test(function() {
        var windowHandler = function () { return "Handler attached to the window"; };
        window[handlerName] = windowHandler;

        var d = (new DOMParser).parseFromString('', 'text/html');
        var b = d.createElement(elementName);
        b[handlerName] = function() { return "Handler attached to windowless element"; };

        assert_equals(window[handlerName], windowHandler);
        assert_equals(b[handlerName], null);

        // Clean up window event handler
        window[handlerName] = null;
      }, "Ignore setting of " + eventName + " window event handlers on windowless " + elementName);
    });

    notShadowedHandlers.forEach(function (eventName) {
      var handlerName = "on" + eventName;

      test(function() {
        var windowHandler = function () { return "Handler attached to the window"; };
        window[handlerName] = windowHandler;

        var d = (new DOMParser).parseFromString('', 'text/html');
        var b = d.createElement(elementName);

        assert_equals(b[handlerName], null);

        var elementHandler = function () { return "Handler attached to the element"; };
        b[handlerName] = elementHandler;

        assert_equals(window[handlerName], windowHandler);
        assert_equals(b[handlerName], elementHandler);

        // Clean up window event handler
        window[handlerName] = null;
      }, eventName + " is unaffected on a windowless " + elementName);
    });
  });

  done();
});
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
        "byte_end": 46,
        "byte_start": 39,
        "col": 1,
        "line": 3
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 109,
        "byte_start": 69,
        "col": 1,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 118,
        "byte_start": 109,
        "col": 41,
        "line": 5
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 174,
        "byte_start": 165,
        "col": 47,
        "line": 6
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 225,
        "byte_start": 216,
        "col": 42,
        "line": 7
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 281,
        "byte_start": 272,
        "col": 47,
        "line": 8
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 2517,
        "byte_start": 290,
        "col": 9,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 2526,
        "byte_start": 2517,
        "col": 1,
        "line": 71
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
  "source_name": "html/webappapis/scripting/events/event-handler-attributes-windowless-body.html"
}
```
