# html/webappapis/scripting/events/onerroreventhandler-frame.html

Counts:
- errors: 4
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/onerroreventhandler-frame.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<body></body>
<script>
function check1(args, callee) {
  parent.t.step(function() {
    parent.assert_equals(callee.length, 5);
    parent.assert_equals(args.length, 5);
    parent.assert_equals(args[0], reference_error.message);
    parent.assert_equals(args[1], reference_error.filename);
    parent.assert_equals(args[2], reference_error.lineno);
    parent.assert_equals(args[3], reference_error.colno);
    parent.assert_equals(args[4], reference_error.error);
    parent.t.done();
  });
}

var reference_error = new ErrorEvent("error", {
  filename: "error_file.js",
  lineno: 333,
  colno: 999,
  message: "there was an error",
  error: {nondefault: 'some unusual object'},
});

parent.t.step(function() {
  document.body.outerHTML = "<body onerror='check1(arguments, arguments.callee)'></body>"
  window.dispatchEvent(reference_error);
});

function check2(args, callee) {
  parent.t2.step(function() {
    parent.assert_equals(callee.length, 5);
    parent.assert_equals(args.length, 1);
    parent.assert_false(args[0] instanceof ErrorEvent);
    parent.t2.done()
  });
}

parent.t2.step(function() {
  document.body.outerHTML = "<body onerror='check2(arguments, arguments.callee)'></body>"
  window.dispatchEvent(new Event("error"));
});

function check3(args, callee) {
  parent.t3.step(function() {
    parent.assert_equals(args.length, 1);
    parent.assert_equals(callee.length, 1);
  });
}

parent.t3.step(function() {
  document.body.outerHTML = "<body><span onerror='check3(arguments, arguments.callee)'></span></body>"
  document.body.firstChild.dispatchEvent(reference_error);
  document.body.firstChild.dispatchEvent(new Event("error"));
  parent.t3.done();
});
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.start_tag_without_doctype",
      "message": "Start tag seen without seeing a doctype first. Expected “<!DOCTYPE html>”.",
      "severity": "Error",
      "span": {
        "byte_end": 6,
        "byte_start": 0,
        "col": 1,
        "line": 1
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_start_tag",
      "message": "Stray start tag “script”.",
      "severity": "Error",
      "span": {
        "byte_end": 22,
        "byte_start": 14,
        "col": 1,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.parser.nonspace_after_body",
      "message": "Non-space character after body.",
      "severity": "Error",
      "span": {
        "byte_end": 1683,
        "byte_start": 22,
        "col": 9,
        "line": 2
      }
    },
    {
      "category": "Html",
      "code": "html.parser.end_tag_after_body_closed",
      "message": "Saw an end tag after “body” had been closed.",
      "severity": "Error",
      "span": {
        "byte_end": 1692,
        "byte_start": 1683,
        "col": 1,
        "line": 56
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
  "source_name": "html/webappapis/scripting/events/onerroreventhandler-frame.html"
}
```
