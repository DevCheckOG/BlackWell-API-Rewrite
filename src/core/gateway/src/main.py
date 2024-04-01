import fastapi
import uvicorn

import datetime

from gateway import Gateway, GATEWAY_CONEXIONS

GATEWAY_API: fastapi.FastAPI = fastapi.FastAPI(
    title="BlackWell Gateway",
    version="1.0.0",
    docs_url=None,
    redoc_url=None,
)


@GATEWAY_API.websocket("/gateway")
async def gateway(
    websocket: fastapi.WebSocket, email: str | None, password: str | None
) -> None:

    if email is None or password is None:

        await websocket.accept()
        await websocket.send_json(
            {
                "title": "BlackWell API - Bad Connection to the Gateway",
                "message": "Please provide an email and a password.",
                "success": False,
                "date": datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
            }
        )

        await websocket.close()
        return

    elif not any(
        email == conexion["email"] and password == conexion["password"]
        for conexion in GATEWAY_CONEXIONS
    ):

        await websocket.accept()
        await websocket.send_json(
            {
                "title": "BlackWell API - Bad Connection to the Gateway",
                "message": "The credentials are not valid.",
                "success": False,
                "date": datetime.datetime.now().strftime("%Y-%m-%d %H:%M:%S"),
            }
        )

        await websocket.close()
        return

    Gateway.connect(email, password, websocket)

    try:

        while True:

            if websocket.client_state == fastapi.websockets.WebSocketState.DISCONNECTED:
                Gateway.disconnect(websocket)
                break

            await websocket.receive()

    except:

        Gateway.disconnect(websocket)


if __name__ == "__main__":

    uvicorn.run(GATEWAY_API, ws_max_queue=64, workers=4)
