using UnityEngine;
using UnityEngine.Events;

public class LoopDetector : MonoBehaviour
{
    [Header("References")]
    public OuroborosController controller;

    [Header("Loop Settings")]
    public int minSegmentsForValidLoop = 10; // prevent premature trigger
    public float loopCooldown = 1.5f;

    [Header("Events")]
    public UnityEvent OnLoopCompleted;

    private float cooldownTimer = 0f;

    void Update()
    {
        if (cooldownTimer > 0f)
            cooldownTimer -= Time.deltaTime;
    }

    // Attach this to SnakeHead — called when head enters tail trigger
    void OnTriggerEnter(Collider other)
    {
        if (cooldownTimer > 0f) return;

        SnakeTail tail = other.GetComponent<SnakeTail>();
        if (tail == null || !tail.isLoopTarget) return;

        int segCount = controller.transform.childCount;
        if (segCount < minSegmentsForValidLoop) return;

        cooldownTimer = loopCooldown;
        Debug.Log("Loop completed!");
        OnLoopCompleted?.Invoke();

        // Reward: grow the snake
        controller.AddSegment();
        controller.AddSegment();

        GameManager.Instance?.RegisterLoop();
    }
}
