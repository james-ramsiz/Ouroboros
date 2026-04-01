using UnityEngine;
using TMPro;

public class GameManager : MonoBehaviour
{
    public static GameManager Instance { get; private set; }

    [Header("UI")]
    public TextMeshProUGUI loopCountText;

    private int loopCount = 0;

    public enum GameState { Playing, Paused, GameOver }
    public GameState State { get; private set; } = GameState.Playing;

    void Awake()
    {
        if (Instance != null && Instance != this) { Destroy(gameObject); return; }
        Instance = this;
    }

    public void RegisterLoop()
    {
        loopCount++;
        if (loopCountText != null)
            loopCountText.text = $"Loops: {loopCount}";

        Debug.Log($"Total loops: {loopCount}");
    }

    public void SetState(GameState newState)
    {
        State = newState;
        Time.timeScale = newState == GameState.Paused ? 0f : 1f;
    }
}
Normal Vector(Space: World)
  └─┐
View Direction(Space: World)
  └─┘
     └─ Dot Product
          └─ One Minus              ← Fresnel: bright at glancing angles
               └─ Power(Exp: 3.0)  ← tighten the rim width
                    └─ Multiply(B: IridescenceIntensity)
                         └─┐
                           ├─ Multiply(B: IridescenceColor) → Emission port
                           └─ Add(B: Smoothness base 0.7)   → Smoothness port
