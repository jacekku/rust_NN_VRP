import math
import tkinter
import json
from tkinter.constants import ALL
from tkinter.filedialog import askopenfilename


def scale(n, start1, stop1, start2, stop2):
    return ((n - start1) / (stop1 - start1)) * (stop2 - start2) + start2


def getCurrentRoute():
    return resultData["route"]


colors = [
    "red",
    "green",
    "blue",
    "magenta",
    "black",
    "orange",
    "brown",
    "teal",
    "maroon",
    "silver",
    "yellow",
    "darkblue"
]
routeIndex = 0
paused = True


def showBest():
    global routeIndex, resultData, paused
    paused = True
    routeIndex = len(resultData["route"]) - 1
    C.delete(ALL)
    draw()
    pass


def reset():
    global routeIndex, resultData, paused
    routeIndex = 0
    C.delete(ALL)
    draw()


def toggleEnded():
    global paused
    paused = not paused
    C.delete(ALL)
    draw()


def nextGlobalStep(clicked=False):
    global routeIndex, resultData, paused
    if (paused):
        C.delete(ALL)
        draw()
        if (not clicked):
            return
    routeIndex += 1

    if (routeIndex >= len(getCurrentRoute())):
        routeIndex = 0
    C.delete(ALL)
    draw()


city_coords = {}
resultData = {'route': [], 'cities': []}
filename = "results.json"
with open(filename, "r", encoding="UTF-8") as cities_file:
    resultData = json.load(cities_file)
    city_coords = resultData['coords']
    iterationIndex, antIndex, routeIndex = 0, 0, 0


def changeFile():
    global filename, resultData, routeIndex, city_coords
    filename = askopenfilename()
    with open(filename, "r", encoding="UTF-8") as result_file:
        resultData = json.load(result_file)
        print(resultData)
        city_coords = resultData['cities']
    routeIndex = 0


top = tkinter.Tk()
width, height = 800, 600

buttonFrame = tkinter.Frame(top)
tkinter.Button(
    buttonFrame,
    text="Next Step",
    command=lambda: nextGlobalStep(True)).grid(
        row=0,
    column=2)
tkinter.Button(
    buttonFrame,
    text="Change file",
    command=changeFile).grid(
        row=1,
    column=0)
tkinter.Button(
    buttonFrame,
    text="Show Best",
    command=showBest).grid(
        row=1,
    column=1)
tkinter.Button(
    buttonFrame,
    text="Start/Stop",
    command=toggleEnded).grid(
        row=1,
    column=2)
tkinter.Button(buttonFrame, text="Reset", command=reset).grid(row=1, column=3)

speedFrame = tkinter.Frame(top)

tkinter.Entry(speedFrame,).grid(row=0, column=0)
tkinter.Button(speedFrame, text="Change speed").grid(row=0, column=1)

buttonFrame.pack()
speedFrame.pack()


def frameStep():
    nextGlobalStep()
    frame.after(10, frameStep)


frame = tkinter.Frame(top)
frame.pack()
frame.after(50, frameStep)

labels = {
    "route": tkinter.Label(top, text="Current route step: " + str(routeIndex)),
}

for label in labels:
    labels[label].pack()


C = tkinter.Canvas(top, bg="white", height=height, width=width)


def find_borders(coords):
    largestX = max(coords, key=lambda v: v['x'])['x']
    largestY = max(coords, key=lambda v: v['y'])['y']
    smallestX = min(coords, key=lambda v: v['x'])['x']
    smallestY = min(coords, key=lambda v: v['y'])['y']
    return largestX, smallestX, largestY, smallestY


def drawPoints():
    C.delete(ALL)
    global truckId
    truckId = 0
    route = getCurrentRoute()
    largestX, smallestX, largestY, smallestY = find_borders(city_coords)
    for i in range(routeIndex):
        coord1 = scale(city_coords[int(route[i]) - 1]['x'], smallestX, largestX, 20, width - 20) + 5, scale(
            city_coords[int(route[i]) - 1]['y'], smallestY, largestY, 20, height - 20) + 5
        coord2 = scale(city_coords[int(route[i + 1]) - 1]['x'], smallestX, largestX, 20, width - 20) + 5, scale(
            city_coords[int(route[i + 1]) - 1]['y'], smallestY, largestY, 20, height - 20) + 5
        C.create_line(coord1, coord2, fill=colors[truckId], width=3)
        if (route[i + 1] == "1"):  # TODO hard coded depot, previously Kraków
            truckId += 1

    for city in city_coords:
        X, Y = city['x'], city['y']
        X, Y = scale(X, smallestX, largestX, 20, width -
                     20), scale(Y, smallestY, largestY, 20, height - 20)
        coord = X, Y, X + 7, Y + 7
        C.create_rectangle(coord, fill="black")
        C.create_text((coord[0], coord[1] - 10), text=city['id'], fill="red")


def draw():
    global routeIndex
    a = {
        "route": routeIndex
    }
    for label in labels:
        labels[label].config(text=label + " " + str(a[label]))
    drawPoints()


draw()

C.pack()
top.mainloop()
