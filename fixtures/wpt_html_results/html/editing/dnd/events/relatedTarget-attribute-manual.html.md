# html/editing/dnd/events/relatedTarget-attribute-manual.html

Counts:
- errors: 0
- warnings: 4
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/relatedTarget-attribute-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
    <head>
        <title>relatedTarget attribute for dragenter and dragleave events</title>
        <meta name="viewport" content="width=device-width">
        <script src="/resources/testharness.js"></script>
        <script src="/resources/testharnessreport.js"></script>
        <style>
        #outerdiv {
          padding: 50px;
          background: blue;
        }
        #innerdiv {
          width:200px;
          height:100px;
          background: green;
        }
        </style>
        <script>
            var drag_test = async_test("dragenter and dragleave are correctly fired.");
            var got_dragenter = false;
            var got_dragleave = false;
            function run() {
                var draggable = document.getElementById("draggable");
                var innerdiv = document.getElementById("innerdiv");
                draggable.addEventListener("dragstart", (e) => {
                    e.dataTransfer.setData("text", draggable.innerHTML);
                });
                innerdiv.addEventListener("dragenter", (e) => {
                    if (!got_dragenter) {
                        test(function() {
                            assert_equals(e.relatedTarget.id, "outerdiv", "dragenter event should have the correct relatedTarget.");
                        }, "dragenter event should have the correct relatedTarget.");
                        got_dragenter = true;
                    }
                });
                innerdiv.addEventListener("dragleave", (e) => {
                    if (!got_dragleave) {
                        test(function() {
                            assert_equals(e.relatedTarget.id, "outerdiv", "dragleave event should have the correct relatedTarget.");
                        }, "dragleave event should have the correct relatedTarget.");
                        got_dragleave = true;
                        if (got_dragenter)
                            drag_test.done();
                    }
                });
            }
        </script>
    </head>
    <body onload="run()">
        <h1>Drag & Drop: relatedTarget attribute for dragenter and dragleave events</h1>
        <h2 id="pointerTypeDescription"></h2>
        <h4>Test Description:
            <ol>
                 <li>Drag the text into the green box.</li>
                 <li>Without releasing the drag, drag the text out of the green box.</li>
            </ol>
        </h4>
        <br>
        <div id="draggable" draggable="true">Drag this text</br>over the green box</div>
        <div id="outerdiv">
          <div id="innerdiv"></div>
        </div>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.heading.empty",
      "message": "Empty heading.",
      "severity": "Warning",
      "span": {
        "byte_end": 2231,
        "byte_start": 2226,
        "col": 41,
        "line": 52
      }
    },
    {
      "category": "Html",
      "code": "html.heading.skip_level",
      "message": "The heading “h4” (with computed level 4) follows the heading “h2” (with computed level 2), skipping 1 heading level.",
      "severity": "Warning",
      "span": {
        "byte_end": 2244,
        "byte_start": 2240,
        "col": 9,
        "line": 53
      }
    },
    {
      "category": "Html",
      "code": "html.void_element.end_tag",
      "message": "End tag “br”.",
      "severity": "Warning",
      "span": {
        "byte_end": 2538,
        "byte_start": 2533,
        "col": 60,
        "line": 60
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
  "source_name": "html/editing/dnd/events/relatedTarget-attribute-manual.html"
}
```
