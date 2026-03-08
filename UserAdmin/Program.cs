using Microsoft.AspNetCore.Identity;
using Microsoft.EntityFrameworkCore;
using Microsoft.Extensions.Configuration;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using mvc.Data;

namespace UserAdmin;

class Program
{
    static async Task Main(string[] args)
    {
        // Build configuration
        // Assumes running from solution root or project root where 'mvc' folder is visible sibling or child.
        // We will check a few locations.
        string appSettingsPath = Path.Combine(Directory.GetCurrentDirectory(), "mvc", "appsettings.json");
        if (!File.Exists(appSettingsPath))
        {
             appSettingsPath = Path.Combine(Directory.GetCurrentDirectory(), "..", "mvc", "appsettings.json");
        }
        
        if (!File.Exists(appSettingsPath))
        {
            Console.WriteLine($"Error: Could not find appsettings.json at {appSettingsPath}. Please run from solution root.");
            return;
        }

        var configuration = new ConfigurationBuilder()
            .AddJsonFile(appSettingsPath, optional: false, reloadOnChange: true)
            .Build();

        var services = new ServiceCollection();

        // Logging
        services.AddLogging(configure => configure.AddConsole());

        // Database
        var connectionString = configuration.GetConnectionString("DefaultConnection");
        if (string.IsNullOrEmpty(connectionString))
        {
            Console.WriteLine("Error: Connection string 'DefaultConnection' not found.");
            return;
        }

        services.AddDbContext<ApplicationDbContext>(options =>
            options.UseNpgsql(connectionString));

        services.AddDataProtection();

        // Identity
        services.AddIdentityCore<IdentityUser>(options => 
        {
            options.SignIn.RequireConfirmedAccount = true;
            options.Password.RequireDigit = false;
            options.Password.RequireLowercase = false;
            options.Password.RequireUppercase = false;
            options.Password.RequireNonAlphanumeric = false;
            options.Password.RequiredLength = 3; // Easy for testing
        })
        .AddEntityFrameworkStores<ApplicationDbContext>()
        .AddDefaultTokenProviders();

        var serviceProvider = services.BuildServiceProvider();

        // Run user creation loop
        while (true)
        {
            Console.WriteLine("\n--- User Administration ---");
            Console.WriteLine("1. Create User");
            Console.WriteLine("2. List Users");
            Console.WriteLine("3. Exit");
            Console.Write("Select an option: ");
            
            var choice = Console.ReadLine();
            
            if (choice == "3") break;

            using (var scope = serviceProvider.CreateScope())
            {
                var userManager = scope.ServiceProvider.GetRequiredService<UserManager<IdentityUser>>();
                var dbContext = scope.ServiceProvider.GetRequiredService<ApplicationDbContext>();

                try
                {
                    if (choice == "1")
                    {
                        await CreateUserAsync(userManager, dbContext);
                    }
                    else if (choice == "2")
                    {
                        await ListUsersAsync(userManager);
                    }
                    else
                    {
                        Console.WriteLine("Invalid option.");
                    }
                }
                catch (Exception ex)
                {
                    Console.WriteLine($"An error occurred: {ex.Message}");
                }
            }
        }
    }

    static async Task CreateUserAsync(UserManager<IdentityUser> userManager, ApplicationDbContext dbContext)
    {
        Console.Write("Enter Username: ");
        var username = Console.ReadLine();

        if (string.IsNullOrWhiteSpace(username))
        {
            Console.WriteLine("Username cannot be empty.");
            return;
        }

        Console.Write("Enter Password: ");
        var password = Console.ReadLine();
        
        if (string.IsNullOrWhiteSpace(password))
        {
            Console.WriteLine("Password cannot be empty.");
            return;
        }

        // Check if user exists
        if (await userManager.FindByNameAsync(username) != null)
        {
             Console.WriteLine("User already exists!");
             return;
        }

        var user = new IdentityUser 
        { 
            UserName = username, 
            Email = $"{username}@example.com", // Dummy email
            EmailConfirmed = true 
        };

        var result = await userManager.CreateAsync(user, password);

        if (result.Succeeded)
        {
            Console.WriteLine($"User '{username}' created successfully!");
        }
        else
        {
            Console.WriteLine("Error creating user:");
            foreach (var error in result.Errors)
            {
                Console.WriteLine($"- {error.Description}");
            }
        }
    }

    static async Task ListUsersAsync(UserManager<IdentityUser> userManager)
    {
        Console.WriteLine("\nExisting Users:");
        // Note: Generic IdentityUser doesn't support IQueryable well without specific store, but UserManager.Users works if store supports IQueryableUserStore
        foreach (var user in userManager.Users)
        {
            Console.WriteLine($"- {user.UserName} (Email: {user.Email})");
        }
        await Task.CompletedTask;
    }
}
