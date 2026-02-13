# html/editing/dnd/events/dragend-event-manual.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/events/dragend-event-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
    <head>
        <title>HTML5 Drag and Drop: Fire dragend event during the drag and drop processing</title>
        <meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/"/>
        <link rel="help" href="http://dev.w3.org/html5/spec/dnd.html#drag-and-drop-processing-model"/>
        <meta name="assert" content="Fire dragend event during the drag and drop processing"/>
        <script type="text/javascript">
            var EVENT, TARGET;

            function DragendEvent(evt)
            {
                if ((TARGET == evt.target) && (EVENT == evt.type))
                {
                  document.getElementById("test_result").firstChild.data = "PASS";
                }
                else
                {
                  document.getElementById("test_result").firstChild.data = "FAIL";
                }
            }

            EVENT = "dragend";

            window.onload = function()
            {
                TARGET = document.getElementById("target");
                TARGET.addEventListener(EVENT, DragendEvent, false);
            }
        </script>
    </head>
    <body>
        <pre>Description: Fire dragend event during the drag and drop processing</pre>
        <table id='testtable' border='1'>
            <tr>
                <td>Test Result</td>
                <td>Test Assertion</td>
            </tr>
            <tr>
                <td id='test_result'>Manual</td>
                <td id='test_assertion'>Test passes if if the word "PASS" appears to the left after following the steps below.
                <div id="manualsteps">
                    Steps:
                    <ol>
                        <li> Drag the blue image
                        <li> Drop it on the green box
                    </ol>
                </div>
            </td>
            </tr>
        </table>
        <p>
        http://dev.w3.org/html5/spec/dnd.html#drag-and-drop-processing-model
        </p>
        <p>
        If the drag operation failed or succeeded, fire a DND event named dragend at the source node.
        </p>
        <img src="/images/blue.png" style="width:200px; height:100px" draggable="true" id="target"/>
        <br /><br />
        <input type="text" style="border:2px green solid; width:200px; height:50px"></input>
    </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.script.type.unnecessary",
      "message": "The “type” attribute is unnecessary for JavaScript resources.",
      "severity": "Warning",
      "span": {
        "byte_end": 527,
        "byte_start": 496,
        "col": 9,
        "line": 9
      }
    },
    {
      "category": "Html",
      "code": "html.img.alt.missing",
      "message": "An “img” element must have an “alt” attribute, except under certain conditions. For details, consult guidance on providing text alternatives for images.",
      "severity": "Warning",
      "span": {
        "byte_end": 2282,
        "byte_start": 2190,
        "col": 9,
        "line": 59
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 2396,
        "byte_start": 2388,
        "col": 85,
        "line": 61
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
  "source_name": "html/editing/dnd/events/dragend-event-manual.html"
}
```
