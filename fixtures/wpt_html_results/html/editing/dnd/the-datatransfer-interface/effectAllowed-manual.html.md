# html/editing/dnd/the-datatransfer-interface/effectAllowed-manual.html

Counts:
- errors: 1
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/the-datatransfer-interface/effectAllowed-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
    <head>
        <title>HTML5 Drag and Drop: Set a value to effectAllowed attribute</title>
        <meta content="text/html; charset=UTF-8" http-equiv="Content-Type"/>
        <link rel="author" title="Microsoft" href="http://www.microsoft.com/"/>
        <link rel="help" href="http://dev.w3.org/html5/spec/dnd.html#datatransfer"/>
        <meta name="assert" content="Set a value to effectAllowed attribute"/>
        <script type="text/javascript">
            var TARGETEVENT1, TARGETEVENT2, TARGET1, TARGET2;

            function DragstartEvent(evt)
            {
                if ((TARGET1 == evt.target) && (TARGETEVENT1 == evt.type))
                {
                    evt.dataTransfer.effectAllowed = "move";
                }
            }
            function DragenterEvent(evt)
            {
                if ((TARGET2 == evt.target) && (TARGETEVENT2 == evt.type))
                {
                    if("move" == evt.dataTransfer.effectAllowed)
                    {
                      document.getElementById("test_result").firstChild.data = "PASS";
                    }
                    else
                    {
                      document.getElementById("test_result").firstChild.data = "FAIL";
                    }
                }
            }

            TARGETEVENT1 = "dragstart";
            TARGETEVENT2 = "dragenter";

            window.onload = function()
            {
                TARGET1 = document.getElementById("target1");
                TARGET2 = document.getElementById("target2");
                TARGET1.addEventListener(TARGETEVENT1, DragstartEvent, false);
                TARGET2.addEventListener(TARGETEVENT2, DragenterEvent, false);
            }
        </script>
    </head>
    <body>
        <pre>Description: Set a value to effectAllowed attribute</pre>
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
                        <li> Drag the blue image and enter the green box
                    </ol>
                </div>
            </td>
            </tr>
        </table>
        <p>
        http://dev.w3.org/html5/spec/dnd.html#datatransfer
        </p>
        <p>
        On setting, if the new value is one of "none", "copy", "copyLink", "copyMove", "link", "linkMove", "move", "all", or "uninitialized", then the attribute's current value must be set to the new value.
        </p>
        <img src="/images/blue.png" style="width:200px; height:100px" draggable="true" id="target1"/>
        <br /><br />
        <input type="text" id="target2" style="border:2px green solid; width:200px; height:50px"></input>
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
        "byte_end": 477,
        "byte_start": 446,
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
        "byte_end": 2887,
        "byte_start": 2794,
        "col": 9,
        "line": 71
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “input”.",
      "severity": "Error",
      "span": {
        "byte_end": 3014,
        "byte_start": 3006,
        "col": 98,
        "line": 73
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
  "source_name": "html/editing/dnd/the-datatransfer-interface/effectAllowed-manual.html"
}
```
